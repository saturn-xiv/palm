use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use hibiscus::cache::redis::Pool as CachePool;
use palm::{
    azalea::v1::{IdRequest, Pager, Pagination},
    camelia::v1,
    to_timestamp, try_grpc, GrpcResult, Thrift,
};
use tonic::{Request, Response};
use validator::Validate;

use super::super::{
    models::leave_word::{Dao as LeaveWordDao, Item as LeaveWord},
    orm::postgresql::Pool as DbPool,
};
use super::{CurrentUserAdapter, Session};

pub struct Service {
    pub loquat: Arc<Thrift>,
    pub gourd: Arc<Thrift>,
    pub db: DbPool,
    pub cache: CachePool,
}

#[tonic::async_trait]
impl v1::leave_word_server::LeaveWord for Service {
    async fn create(&self, req: Request<v1::LeaveWordCreateRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let req = req.into_inner();

        {
            let form = Create { body: &req.body };
            try_grpc!(form.validate())?;
        }

        try_grpc!(LeaveWordDao::create(
            db,
            &ss.lang,
            &ss.client_ip,
            &req.body,
            req.editor(),
        ))?;
        Ok(Response::new(()))
    }

    async fn index(&self, req: Request<Pager>) -> GrpcResult<v1::LeaveWordIndexResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let pager = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        let total = try_grpc!(LeaveWordDao::count(db))?;
        let items = try_grpc!(LeaveWordDao::all(db, pager.offset(total), pager.size()))?;

        Ok(Response::new(v1::LeaveWordIndexResponse {
            items: items.into_iter().map(|x| x.into()).collect(),
            pagination: Some(Pagination::new(&pager, total)),
        }))
    }

    async fn publish(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(LeaveWordDao::publish(db, req.id,))?;
        Ok(Response::new(()))
    }
    async fn destroy(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(LeaveWordDao::destroy(db, req.id))?;
        Ok(Response::new(()))
    }
    async fn close(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(LeaveWordDao::set_status(
            db,
            req.id,
            v1::leave_word_index_response::item::Status::Closed
        ))?;
        Ok(Response::new(()))
    }
    async fn process(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(user.is_administrator(policy))?;
        }

        try_grpc!(LeaveWordDao::set_status(
            db,
            req.id,
            v1::leave_word_index_response::item::Status::Processing
        ))?;
        Ok(Response::new(()))
    }
}

impl From<LeaveWord> for v1::leave_word_index_response::Item {
    fn from(x: LeaveWord) -> Self {
        Self {
            id: x.id,
            lang: x.lang.clone(),
            ip: x.ip.clone(),
            body: x.body.clone(),
            editor: x.body_editor,
            status: x.status,
            deleted_at: x.deleted_at.map(|x| to_timestamp!(x)),
            published_at: x.published_at.map(|x| to_timestamp!(x)),
            updated_at: Some(to_timestamp!(x.updated_at)),
        }
    }
}

#[derive(Validate)]
pub struct Create<'a> {
    #[validate(length(min = 10, max = 1023))]
    pub body: &'a str,
}
