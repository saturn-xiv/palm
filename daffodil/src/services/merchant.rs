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
        book::{Dao as BookDao, Item as Book},
        merchant::Dao as MerchantDao,
    },
    NAME,
};
use super::{new_book_detail, new_merchant_item};

pub struct Service {
    pub loquat: Arc<Thrift>,
    pub gourd: Arc<Thrift>,
    pub jasmine: Arc<Thrift>,
    pub db: DbPool,
    pub cache: CachePool,
}

#[tonic::async_trait]
impl v1::merchant_server::Merchant for Service {
    async fn create(&self, req: Request<v1::MerchantCreateRequest>) -> GrpcResult<()> {
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
                description: req.description.as_deref(),
                address: req.address.as_deref(),
                contact: req.contact.as_deref(),
            };
            try_grpc!(form.validate())?;
        }

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        {
            let book = try_grpc!(BookDao::by_id(db, req.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            MerchantDao::create(
                db,
                user.id,
                req.book_id,
                &req.name,
                (
                    req.address.as_deref(),
                    req.contact.as_deref(),
                    req.description.as_deref(),
                ),
                req.cover,
            )?;
            LogDao::add::<_, Book>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(req.book_id),
                &format!("create merchant({})", req.name),
            )?;
            Ok(())
        }))?;

        Ok(Response::new(()))
    }
    async fn update(&self, req: Request<v1::MerchantUpdateRequest>) -> GrpcResult<()> {
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
                description: req.description.as_deref(),
                address: req.address.as_deref(),
                contact: req.contact.as_deref(),
            };
            try_grpc!(form.validate())?;
        }

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        let merchant = try_grpc!(MerchantDao::by_id(db, req.id))?;
        if merchant.user_id != user.id {
            return Err(Status::permission_denied(""));
        }
        {
            let book = try_grpc!(BookDao::by_id(db, merchant.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            MerchantDao::update(
                db,
                req.id,
                &req.name,
                req.address.as_deref(),
                req.contact.as_deref(),
                req.description.as_deref(),
                req.cover,
            )?;
            LogDao::add::<_, Book>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(merchant.book_id),
                &format!("change merchant({} => {})", merchant.name, req.name),
            )?;
            Ok(())
        }))?;

        Ok(Response::new(()))
    }

    async fn by_book(&self, req: Request<IdRequest>) -> GrpcResult<v1::MerchantIndexResponse> {
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
        for it in try_grpc!(MerchantDao::by_book(db, req.id))?.iter() {
            let it = try_grpc!(new_merchant_item(db, s3, it))?;
            items.push(it);
        }

        Ok(Response::new(v1::MerchantIndexResponse { items }))
    }
    async fn show(&self, req: Request<IdRequest>) -> GrpcResult<v1::MerchantShowResponse> {
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
        let merchant = try_grpc!(MerchantDao::by_id(db, req.id))?;

        let book = try_grpc!(BookDao::by_id(db, merchant.book_id))?;
        try_grpc!(book.can_manage(policy, &user))?;

        let item = try_grpc!(new_merchant_item(db, s3, &merchant))?;
        let book = try_grpc!(new_book_detail(db, s3, &book))?;

        Ok(Response::new(v1::MerchantShowResponse {
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
        let merchant = try_grpc!(MerchantDao::by_id(db, req.id))?;
        if merchant.deleted_at.is_none() {
            return Err(Status::resource_exhausted("this merchant isn't disabled"));
        }
        {
            let book = try_grpc!(BookDao::by_id(db, merchant.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            MerchantDao::enable(db, req.id, true)?;
            LogDao::add::<_, Book>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(merchant.book_id),
                &format!("enable merchant({})", merchant.name),
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
        let merchant = try_grpc!(MerchantDao::by_id(db, req.id))?;
        if merchant.deleted_at.is_some() {
            return Err(Status::resource_exhausted(
                "this merchant is already disabled",
            ));
        }
        {
            let book = try_grpc!(BookDao::by_id(db, merchant.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            MerchantDao::enable(db, req.id, false)?;
            LogDao::add::<_, Book>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(merchant.id),
                &format!("disable merchant({})", merchant.name),
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
    pub description: Option<&'a str>,
    #[validate(length(min = 1, max = 255))]
    pub address: Option<&'a str>,
    #[validate(length(min = 1, max = 255))]
    pub contact: Option<&'a str>,
}
