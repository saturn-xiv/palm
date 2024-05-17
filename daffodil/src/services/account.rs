use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use camelia::{
    models::log::Dao as LogDao, orm::postgresql::Pool as DbPool, services::CurrentUserAdapter,
};
use diesel::Connection as DieselConntection;
use hibiscus::{cache::redis::Pool as CachePool, session::Session};
use palm::{
    azalea::v1::IdRequest, camelia::v1::user_logs_response::item::Level as LogLevel, daffodil::v1,
    try_grpc, Error, GrpcResult, Thrift,
};
use tonic::{Request, Response, Status};
use validator::Validate;

use super::super::{
    models::{
        account::{Dao as AccountDao, Item as Account},
        book::{Dao as BookDao, Item as Book},
    },
    NAME,
};
use super::{new_account_item, new_book_detail};

pub struct Service {
    pub loquat: Arc<Thrift>,
    pub gourd: Arc<Thrift>,
    pub jasmine: Arc<Thrift>,
    pub db: DbPool,
    pub cache: CachePool,
}

#[tonic::async_trait]
impl v1::account_server::Account for Service {
    async fn create(&self, req: Request<v1::AccountCreateRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let form = Form {
                name: &req.name,
                description: &req.description,
            };
            try_grpc!(form.validate())?;
        }

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        {
            let book = try_grpc!(BookDao::by_id(db, req.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            AccountDao::create(
                db,
                user.id,
                req.book_id,
                &req.name,
                req.r#type(),
                &req.description,
                req.cover_id,
            )?;
            LogDao::add::<_, Book>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(req.book_id),
                &format!("create account({})", req.name),
            )?;
            Ok(())
        }))?;

        Ok(Response::new(()))
    }
    async fn update(&self, req: Request<v1::AccountUpdateRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let form = Form {
                name: &req.name,
                description: &req.description,
            };
            try_grpc!(form.validate())?;
        }

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        let account = try_grpc!(AccountDao::by_id(db, req.id))?;
        if account.user_id != user.id {
            return Err(Status::permission_denied(""));
        }
        {
            let book = try_grpc!(BookDao::by_id(db, account.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            AccountDao::update(db, req.id, &req.name, &req.description, req.cover_id)?;
            LogDao::add::<_, Book>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(account.book_id),
                &format!("change account({} => {})", account.name, req.name),
            )?;
            Ok(())
        }))?;

        Ok(Response::new(()))
    }

    async fn by_book(&self, req: Request<IdRequest>) -> GrpcResult<v1::AccountIndexResponse> {
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
        {
            let book = try_grpc!(BookDao::by_id(db, req.id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        let mut items = Vec::new();
        for it in try_grpc!(AccountDao::by_book(db, req.id))?.iter() {
            let it = try_grpc!(new_account_item(db, s3, it))?;
            items.push(it);
        }

        Ok(Response::new(v1::AccountIndexResponse { items }))
    }
    async fn show(&self, req: Request<IdRequest>) -> GrpcResult<v1::AccountShowResponse> {
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
        let account = try_grpc!(AccountDao::by_id(db, req.id))?;

        let book = try_grpc!(BookDao::by_id(db, account.book_id))?;
        try_grpc!(book.can_manage(policy, &user))?;

        let item = try_grpc!(new_account_item(db, s3, &account))?;
        let book = try_grpc!(new_book_detail(db, s3, &book))?;

        Ok(Response::new(v1::AccountShowResponse {
            item: Some(item),
            book: Some(book),
        }))
    }

    async fn enable(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        let account = try_grpc!(AccountDao::by_id(db, req.id))?;
        if account.deleted_at.is_none() {
            return Err(Status::resource_exhausted("this account isn't disabled"));
        }
        {
            let book = try_grpc!(BookDao::by_id(db, account.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            AccountDao::enable(db, req.id, true)?;
            LogDao::add::<_, Account>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(account.id),
                &format!("enable account({})", account.name),
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
        let policy = self.gourd.deref();
        let req = req.into_inner();

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        let account = try_grpc!(AccountDao::by_id(db, req.id))?;
        if account.deleted_at.is_some() {
            return Err(Status::resource_exhausted(
                "this account is already disabled",
            ));
        }
        {
            let book = try_grpc!(BookDao::by_id(db, account.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            AccountDao::enable(db, req.id, false)?;
            LogDao::add::<_, Account>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(account.id),
                &format!("disable account({})", account.name),
            )?;
            Ok(())
        }))?;

        Ok(Response::new(()))
    }
}

#[derive(Validate)]
struct Form<'a> {
    #[validate(length(min = 1, max = 63))]
    pub name: &'a str,
    #[validate(length(min = 1, max = 511))]
    pub description: &'a str,
}
