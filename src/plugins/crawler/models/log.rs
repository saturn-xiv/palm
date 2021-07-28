use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use juniper::GraphQLObject;
use serde::Serialize;

use super::super::super::super::{orm::postgresql::Connection, Result};
use super::schema::crawler_logs;

#[derive(Queryable, Serialize, GraphQLObject)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub url: String,
    pub body: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(&self, url: &str, body: &str) -> Result<()>;
    fn all(&self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count(&self) -> Result<i64>;
    fn latest(&self, url: &str) -> Result<Item>;
}

impl Dao for Connection {
    fn add(&self, url: &str, body: &str) -> Result<()> {
        insert_into(crawler_logs::dsl::crawler_logs)
            .values((
                crawler_logs::dsl::url.eq(url),
                crawler_logs::dsl::body.eq(body),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = crawler_logs::dsl::crawler_logs
            .order(crawler_logs::dsl::created_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn count(&self) -> Result<i64> {
        let it = crawler_logs::dsl::crawler_logs.count().first(self)?;
        Ok(it)
    }
    fn latest(&self, url: &str) -> Result<Item> {
        let it = crawler_logs::dsl::crawler_logs
            .filter(crawler_logs::dsl::url.eq(url))
            .order(crawler_logs::dsl::created_at.desc())
            .first(self)?;
        Ok(it)
    }
}
