use chrono::NaiveDateTime;
use hibiscus::{
    cache::redis::ClusterConnection as Cache,
    jwt::Jwt,
    pagination::{Pager, Pagination},
    session::Session,
    Result,
};
use juniper::GraphQLObject;

use super::super::{
    models::log::{Dao as LogDao, Item as Log},
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};

#[derive(GraphQLObject)]
#[graphql(name = "LogIndexResponseItem")]
pub struct IndexResponseItem {
    pub id: i32,
    pub plugin: String,
    pub level: String,
    pub ip: String,
    pub resource_type: String,
    pub resource_id: Option<i32>,
    pub message: String,
    pub created_at: NaiveDateTime,
}

impl From<Log> for IndexResponseItem {
    fn from(x: Log) -> Self {
        Self {
            id: x.id,
            plugin: x.plugin.clone(),
            level: x.level.clone(),
            ip: x.ip.clone(),
            message: x.message.clone(),
            resource_type: x.resource_type.clone(),
            resource_id: x.resource_id,
            created_at: x.created_at,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "LogIndexResponse")]
pub struct IndexResponse {
    pub items: Vec<IndexResponseItem>,
    pub pagination: Pagination,
}

impl IndexResponse {
    pub fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        jwt: &J,
        pager: &Pager,
    ) -> Result<Self> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        let total = LogDao::count(db, user.id)?;
        let items = LogDao::all(db, user.id, pager.offset(total), pager.size())?
            .into_iter()
            .map(|x| x.into())
            .collect();

        Ok(Self {
            items,
            pagination: Pagination::new(pager, total),
        })
    }
}
