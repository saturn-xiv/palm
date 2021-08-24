use std::ops::Deref;
use std::sync::Arc;

use tonic::{Request, Response, Status};

use super::super::super::{jwt::Jwt, orm::postgresql::Pool as DbPool, GrpcResult};
use super::super::nut::{
    models::user::CurrentUser,
    services::Session,
    v1::{Page, Pagination},
};
use super::{
    models::log::{Dao as LogDao, Item as Log},
    v1::{crawler_server::Crawler, logs_response, LogsResponse},
};

impl From<Log> for logs_response::Item {
    fn from(it: Log) -> Self {
        Self {
            id: it.id,
            url: it.url.clone(),
            body: it.body.clone(),
            created_at: Some(to_timestamp!(it.created_at)),
        }
    }
}

pub struct Service {
    pub jwt: Arc<Jwt>,
    pub db: DbPool,
}

#[tonic::async_trait]
impl Crawler for Service {
    async fn logs(&self, req: Request<Page>) -> GrpcResult<Response<LogsResponse>> {
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
