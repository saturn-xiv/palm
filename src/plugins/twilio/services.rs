use std::ops::Deref;
use std::sync::Arc;

use tonic::{Request, Response, Status};

use super::super::super::{
    auth::{models::user::CurrentUser, services::Session},
    jwt::Jwt,
    orm::postgresql::Pool as DbPool,
    GrpcResult,
};
use super::super::nut::v1::{Page, Pagination};
use super::{
    models::log::{Dao as LogDao, Item as Log},
    protocols::Config,
    v1::{logs_response, twilio_server::Twilio, LogsResponse, TextMessage},
};

impl From<Log> for logs_response::Item {
    fn from(it: Log) -> Self {
        Self {
            id: it.id,
            from: it.from.clone(),
            to: it.to.clone(),
            body: it.body.clone(),
            created_at: Some(to_timestamp!(it.created_at)),
        }
    }
}

pub struct Service {
    pub jwt: Arc<Jwt>,
    pub twilio: Arc<Config>,
    pub db: DbPool,
}

#[tonic::async_trait]
impl Twilio for Service {
    async fn send(&self, req: Request<TextMessage>) -> GrpcResult<()> {
        let user = current_user!(self, &req);
        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        try_grpc!(user.is_administrator(db))?;
        Ok(Response::new(()))
    }
    async fn logs(&self, req: Request<Page>) -> GrpcResult<LogsResponse> {
        let user = current_user!(self, &req);
        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        try_grpc!(user.is_administrator(db))?;
        let req = req.into_inner();
        let total = try_grpc!(LogDao::count(db))?;
        let page = req.validate(total);
        Ok(Response::new(LogsResponse {
            pagination: Some(Pagination {
                total,
                size: page.size,
                index: page.index,
            }),
            items: try_grpc!(LogDao::all(db, page.offset(), page.size))?
                .iter()
                .map(|x| x.clone().into())
                .collect(),
        }))
    }
}
