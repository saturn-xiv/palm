use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use serde::Serialize;

use super::super::super::super::{orm::postgresql::Connection, Result};
use super::schema::crawler_logs;

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub site_id: i64,
    pub body: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(&self, site: i64, body: &str) -> Result<()>;
    fn all(&self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count(&self) -> Result<i64>;
    fn latest(&self, site: i64) -> Result<Item>;
}

impl Dao for Connection {
    fn add(&self, site: i64, body: &str) -> Result<()> {
        insert_into(crawler_logs::dsl::crawler_logs)
            .values((
                crawler_logs::dsl::site_id.eq(site),
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
    fn latest(&self, site: i64) -> Result<Item> {
        let it = crawler_logs::dsl::crawler_logs
            .filter(crawler_logs::dsl::site_id.eq(site))
            .order(crawler_logs::dsl::created_at.desc())
            .first(self)?;
        Ok(it)
    }
}
