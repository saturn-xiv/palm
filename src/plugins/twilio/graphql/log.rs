use juniper::GraphQLObject;

use super::super::super::super::{jwt::Jwt, orm::postgresql::Connection as Db, Result};
use super::super::super::nut::graphql::{Page, Pagination, Session};
use super::super::models::log::{Dao as LogDao, Item as Log};

#[derive(GraphQLObject)]
#[graphql(name = "SmsLogs")]
pub struct Logs {
    pub items: Vec<Log>,
    pub pagination: Pagination,
}

impl Logs {
    pub fn new(ss: &Session, page: &Page, db: &Db, jwt: &Jwt) -> Result<Self> {
        let user = ss.current_user(db, jwt)?;
        user.is_administrator(db)?;
        let total = LogDao::count(db)?;
        let (offset, limit) = page.pagination(total);
        let it = Self {
            items: LogDao::all(db, offset, limit)?,
            pagination: Pagination::new(total, page),
        };
        Ok(it)
    }
}
