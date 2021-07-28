use chrono::NaiveDateTime;
use diesel::{delete, insert_into, prelude::*};
use juniper::GraphQLObject;
use serde::Serialize;

use super::super::super::super::{orm::postgresql::Connection, Result};
use super::schema::crawler_sites;

#[derive(Queryable, Serialize, GraphQLObject)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub ttl: i32,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(&self, name: &str, url: &str) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn count(&self) -> Result<i64>;
    fn destory(&self, id: i32) -> Result<()>;
    fn by_id(&self, id: i32) -> Result<Item>;
}

impl Dao for Connection {
    fn add(&self, name: &str, url: &str) -> Result<()> {
        insert_into(crawler_sites::dsl::crawler_sites)
            .values((
                crawler_sites::dsl::url.eq(url),
                crawler_sites::dsl::name.eq(name),
            ))
            .execute(self)?;
        Ok(())
    }
    fn all(&self) -> Result<Vec<Item>> {
        let items = crawler_sites::dsl::crawler_sites
            .order(crawler_sites::dsl::name.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn count(&self) -> Result<i64> {
        let it = crawler_sites::dsl::crawler_sites.count().first(self)?;
        Ok(it)
    }
    fn destory(&self, id: i32) -> Result<()> {
        delete(crawler_sites::dsl::crawler_sites.filter(crawler_sites::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&self, id: i32) -> Result<Item> {
        let it = crawler_sites::dsl::crawler_sites
            .filter(crawler_sites::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
}
