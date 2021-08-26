use chrono::NaiveDateTime;
use diesel::{delete, insert_into, prelude::*};
use serde::Serialize;

use super::super::super::super::{orm::postgresql::Connection, Result};
use super::schema::{crawler_logs, crawler_sites};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub cron: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(&self, name: &str, url: &str, cron: &str) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn count(&self) -> Result<i64>;
    fn destory(&self, id: i64) -> Result<()>;
    fn by_id(&self, id: i64) -> Result<Item>;
    fn by_url(&self, url: &str) -> Result<Item>;
}

impl Dao for Connection {
    fn add(&self, name: &str, url: &str, cron: &str) -> Result<()> {
        insert_into(crawler_sites::dsl::crawler_sites)
            .values((
                crawler_sites::dsl::url.eq(url),
                crawler_sites::dsl::name.eq(name),
                crawler_sites::dsl::cron.eq(cron),
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
    fn destory(&self, id: i64) -> Result<()> {
        delete(crawler_logs::dsl::crawler_logs.filter(crawler_logs::dsl::site_id.eq(id)))
            .execute(self)?;
        delete(crawler_sites::dsl::crawler_sites.filter(crawler_sites::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&self, id: i64) -> Result<Item> {
        let it = crawler_sites::dsl::crawler_sites
            .filter(crawler_sites::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_url(&self, url: &str) -> Result<Item> {
        let it = crawler_sites::dsl::crawler_sites
            .filter(crawler_sites::dsl::url.eq(url))
            .first(self)?;
        Ok(it)
    }
}
