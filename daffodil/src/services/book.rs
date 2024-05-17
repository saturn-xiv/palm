use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use camelia::{
    models::{log::Dao as LogDao, user::Item as User},
    orm::postgresql::Pool as DbPool,
    services::CurrentUserAdapter,
};
use diesel::Connection as DieselConntection;
use hibiscus::{cache::redis::Pool as CachePool, session::Session};
use hyper::StatusCode;
use palm::{
    azalea::v1::IdRequest,
    camelia::v1::{user_logs_response::item::Level as LogLevel, Operation},
    daffodil::v1,
    gourd::Policy,
    random::uuid,
    try_grpc, Error, GrpcResult, HttpError, Result, Thrift,
};
use tonic::{Request, Response, Status};
use validator::Validate;

use super::super::{
    models::{
        account::Dao as AccountDao,
        book::{Dao as BookDao, Item as Book},
        merchant::Dao as MerchantDao,
    },
    NAME,
};
use super::{new_account_detail, new_book_item, new_merchant_detail};

pub struct Service {
    pub loquat: Arc<Thrift>,
    pub gourd: Arc<Thrift>,
    pub jasmine: Arc<Thrift>,
    pub db: DbPool,
    pub cache: CachePool,
}

#[tonic::async_trait]
impl v1::book_server::Book for Service {
    async fn create(&self, req: Request<v1::BookCreateRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let req = req.into_inner();

        {
            let form = Form {
                name: &req.name,
                description: &req.description,
            };
            try_grpc!(form.validate())?;
        }

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            let uid = uuid();
            BookDao::create(db, user.id, &uid, &req.name, &req.description, req.cover_id)?;
            let book = BookDao::by_uid(db, &uid)?;
            LogDao::add::<_, Book>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(book.id),
                &format!("create accounting book({})", book.name),
            )?;
            Ok(())
        }))?;

        Ok(Response::new(()))
    }
    async fn update(&self, req: Request<v1::BookUpdateRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let req = req.into_inner();

        {
            let form = Form {
                name: &req.name,
                description: &req.description,
            };
            try_grpc!(form.validate())?;
        }

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;

        let book = try_grpc!(BookDao::by_id(db, req.id))?;
        if book.user_id != user.id {
            return Err(Status::permission_denied(""));
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            BookDao::update(db, req.id, &req.name, &req.description, req.cover_id)?;
            if req.name != book.name {
                LogDao::add::<_, Book>(
                    db,
                    user.id,
                    NAME,
                    LogLevel::Info,
                    &ss.client_ip,
                    Some(req.id),
                    &format!("change accounting book({}=>{})", book.name, req.name),
                )?;
            }
            Ok(())
        }))?;

        Ok(Response::new(()))
    }

    async fn index(&self, req: Request<()>) -> GrpcResult<v1::BookIndexResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let s3 = self.jasmine.deref();

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;

        let mut items = Vec::new();
        for it in try_grpc!(BookDao::by_user(db, user.id))?.iter() {
            let it = try_grpc!(new_book_item(db, s3, it))?;
            items.push(it);
        }

        Ok(Response::new(v1::BookIndexResponse { items }))
    }
    async fn show(&self, req: Request<IdRequest>) -> GrpcResult<v1::BookShowResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let s3 = self.jasmine.deref();
        let req = req.into_inner();

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;

        let book = try_grpc!(BookDao::by_id(db, req.id))?;
        try_grpc!(book.can_manage(policy, &user))?;

        let item = try_grpc!(new_book_item(db, s3, &book))?;
        let mut merchants = Vec::new();
        for it in try_grpc!(MerchantDao::by_book(db, book.id))?.iter() {
            let it = try_grpc!(new_merchant_detail(db, s3, it))?;
            merchants.push(it);
        }
        let mut accounts = Vec::new();
        for it in try_grpc!(AccountDao::by_book(db, book.id))?.iter() {
            let it = try_grpc!(new_account_detail(db, s3, it))?;
            accounts.push(it);
        }
        Ok(Response::new(v1::BookShowResponse {
            item: Some(item),
            merchants,
            accounts,
        }))
    }

    async fn enable(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let req = req.into_inner();

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        let book = try_grpc!(BookDao::by_id(db, req.id))?;
        if user.id != book.user_id {
            return Err(Status::permission_denied("only owner can do this"));
        }
        if book.deleted_at.is_none() {
            return Err(Status::resource_exhausted(
                "this book already isn't disabled",
            ));
        }
        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            BookDao::enable(db, req.id, true)?;
            LogDao::add::<_, Book>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(book.id),
                &format!("enable accounting book({})", book.name),
            )?;
            Ok(())
        }))?;

        Ok(Response::new(()))
    }
    async fn disable(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let req = req.into_inner();

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        let book = try_grpc!(BookDao::by_id(db, req.id))?;
        if user.id != book.user_id {
            return Err(Status::permission_denied("only owner can do this"));
        }
        if book.deleted_at.is_some() {
            return Err(Status::resource_exhausted("this book is already disabled"));
        }
        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            BookDao::enable(db, req.id, false)?;
            LogDao::add::<_, Book>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(book.id),
                &format!("disable accounting book({})", book.name),
            )?;
            Ok(())
        }))?;

        Ok(Response::new(()))
    }
}

impl Book {
    pub fn can_manage<P: Policy>(&self, policy: &P, user: &User) -> Result<()> {
        if self.deleted_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
        }
        if self.user_id == user.id {
            return Ok(());
        }
        if user
            .can::<_, _, Book>(policy, Operation::Manage.as_str_name(), Some(self.id))
            .is_ok()
        {
            return Ok(());
        }
        Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
    }
}

#[derive(Validate)]
struct Form<'a> {
    #[validate(length(min = 1, max = 63))]
    pub name: &'a str,
    #[validate(length(min = 1, max = 511))]
    pub description: &'a str,
}
