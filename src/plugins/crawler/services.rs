use std::ops::Deref;
use std::sync::Arc;

use tonic::{Request, Response, Status};

use super::super::super::{
    jwt::Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    GrpcResult, Result,
};
use super::super::nut::{
    models::user::CurrentUser,
    services::Session,
    v1::{Page, Pagination},
};
use super::{
    models::{log::Dao as LogDao, site::Dao as SiteDao},
    v1::{crawler_server::Crawler, logs_response, LogsResponse},
};

impl LogsResponse {
    pub fn load(&mut self, db: &Db, page: &Page) -> Result<()> {
        let total = LogDao::count(db)?;
        let page = page.validate(total);
        self.pagination = Some(Pagination {
            total,
            size: page.size,
            index: page.index,
        });
        for it in LogDao::all(db, page.offset(), page.size)? {
            let site = SiteDao::by_id(db, it.site_id)?;
            let log = logs_response::Item {
                id: it.id,
                url: site.url,
                body: it.body.clone(),
                created_at: Some(to_timestamp!(it.created_at)),
            };
            self.items.push(log);
        }
        Ok(())
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
        let mut ret = LogsResponse::default();
        try_grpc!(ret.load(db, &req))?;
        Ok(Response::new(ret))
    }
}
