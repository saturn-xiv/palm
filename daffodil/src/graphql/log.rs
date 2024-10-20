use std::ops::DerefMut;

use chrono::NaiveDateTime;
use juniper::GraphQLObject;
use petunia::{
    graphql::{Pager, Pagination, Resource},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    Result,
};

use super::super::{
    models::log::{Dao as LogDao, Item as Log},
    session::current_user,
};

#[derive(GraphQLObject)]
#[graphql(name = "Log")]
pub struct Item {
    pub id: i32,
    pub plugin: String,
    pub ip: String,
    pub level: String,
    pub resource: Resource,
    pub message: String,
    pub created_at: NaiveDateTime,
}

impl From<Log> for Item {
    fn from(it: Log) -> Self {
        Self {
            id: it.id,
            plugin: it.plugin.clone(),
            ip: it.ip.clone(),
            level: it.level.clone(),
            resource: Resource {
                r#type: it.resource_type.clone(),
                id: it.resource_id,
            },
            message: it.message.clone(),
            created_at: it.created_at,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "LogList")]
pub struct List {
    pub pagination: Pagination,
    pub items: Vec<Item>,
}

impl List {
    pub fn new(ss: &Session, db: &DbPool, jwt: &Jwt, pager: &Pager) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
        let mut items = Vec::new();
        let total = LogDao::count_by_user(db, user.id)?;
        let pagination = Pagination::new(pager, total);
        for it in LogDao::index_by_user(db, user.id, pager.offset(total), pager.size())? {
            items.push(it.into());
        }
        Ok(Self { items, pagination })
    }
}
