use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use camelia::{
    models::log::Dao as LogDao, orm::postgresql::Pool as DbPool, services::CurrentUserAdapter,
};
use chrono::{DateTime, Utc};
use diesel::Connection as DieselConntection;
use hibiscus::{cache::redis::Pool as CachePool, session::Session};
use hyper::StatusCode;
use palm::{
    azalea::v1::IdRequest, camelia::v1::user_logs_response::item::Level as LogLevel, daffodil::v1,
    to_datetime, try_grpc, Error, GrpcResult, HttpError, Thrift,
};
use tonic::{Request, Response, Status};
use validator::Validate;

use super::super::{
    models::{
        account::Dao as AccountDao,
        book::{Dao as BookDao, Item as Book},
        merchant::Dao as MerchantDao,
        transaction::{trash::Dao as TransactionTrashDao, Dao as TransactionDao},
    },
    NAME,
};
use super::{new_transaction_item, new_transaction_trash_item};

pub struct Service {
    pub loquat: Arc<Thrift>,
    pub gourd: Arc<Thrift>,
    pub jasmine: Arc<Thrift>,
    pub db: DbPool,
    pub cache: CachePool,
}

#[tonic::async_trait]
impl v1::transaction_server::Transaction for Service {
    async fn create(&self, req: Request<v1::TransactionCreateRequest>) -> GrpcResult<()> {
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
                summary: &req.summary,
            };
            try_grpc!(form.validate())?;
        }
        let paid_at: DateTime<Utc> = try_grpc!(to_datetime!(req.paid_at).ok_or(Box::new(
            HttpError(StatusCode::BAD_REQUEST, Some("bad paid at".to_string()),)
        )))?;
        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        {
            let book = try_grpc!(BookDao::by_id(db, req.book))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            let source_account = AccountDao::by_id(db, req.source_account)?;
            TransactionDao::create(
                db,
                user.id,
                req.book,
                (req.source_account, req.destination_account),
                (req.merchant, req.r#type.try_into()?, paid_at.naive_local()),
                (req.amount, &source_account.currency),
                &req.summary,
            )?;
            Ok(())
        }))?;
        Ok(Response::new(()))
    }

    async fn by_book(&self, req: Request<IdRequest>) -> GrpcResult<v1::TransactionIndexResponse> {
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
        for it in try_grpc!(TransactionDao::by_book(db, req.id))?.iter() {
            let it = try_grpc!(new_transaction_item(db, s3, it))?;
            items.push(it);
        }
        Ok(Response::new(v1::TransactionIndexResponse { items }))
    }
    async fn by_account(
        &self,
        req: Request<IdRequest>,
    ) -> GrpcResult<v1::TransactionIndexResponse> {
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
        {
            let book = try_grpc!(BookDao::by_id(db, account.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }
        let mut items = Vec::new();
        for it in try_grpc!(TransactionDao::by_account(db, req.id))?.iter() {
            let it = try_grpc!(new_transaction_item(db, s3, it))?;
            items.push(it);
        }
        Ok(Response::new(v1::TransactionIndexResponse { items }))
    }
    async fn by_merchant(
        &self,
        req: Request<IdRequest>,
    ) -> GrpcResult<v1::TransactionIndexResponse> {
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
        {
            let book = try_grpc!(BookDao::by_id(db, merchant.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }
        let mut items = Vec::new();
        for it in try_grpc!(TransactionDao::by_merchant(db, req.id))?.iter() {
            let it = try_grpc!(new_transaction_item(db, s3, it))?;
            items.push(it);
        }
        Ok(Response::new(v1::TransactionIndexResponse { items }))
    }

    async fn delete(&self, req: Request<v1::TransactionDeleteRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        {
            let form = Delete {
                reason: &req.reason,
            };
            try_grpc!(form.validate())?;
        }

        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
        let transaction = try_grpc!(TransactionDao::by_id(db, req.id))?;
        if transaction.user_id != user.id {
            return Err(Status::permission_denied(""));
        }
        {
            let book = try_grpc!(BookDao::by_id(db, transaction.book_id))?;
            try_grpc!(book.can_manage(policy, &user))?;
        }

        try_grpc!(db.transaction::<_, Error, _>(move |db| {
            TransactionDao::delete(db, req.id)?;
            TransactionTrashDao::create(db, user.id, &transaction, &req.reason)?;
            LogDao::add::<_, Book>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(transaction.book_id),
                &format!("delete transaction({}, {})", transaction.id, req.reason),
            )?;
            Ok(())
        }))?;
        Ok(Response::new(()))
    }

    async fn trash_by_book(
        &self,
        req: Request<IdRequest>,
    ) -> GrpcResult<v1::TransactionTrashResponse> {
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
        for it in try_grpc!(TransactionTrashDao::by_book(db, req.id))?.iter() {
            let it = try_grpc!(new_transaction_trash_item(db, s3, it))?;
            items.push(it);
        }
        Ok(Response::new(v1::TransactionTrashResponse { items }))
    }
}

#[derive(Validate)]
struct Form<'a> {
    #[validate(length(min = 1, max = 511))]
    summary: &'a str,
}

#[derive(Validate)]
struct Delete<'a> {
    #[validate(length(min = 1, max = 255))]
    reason: &'a str,
}
