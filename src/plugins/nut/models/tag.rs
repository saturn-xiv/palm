use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use serde::Serialize;

use super::super::super::super::{orm::postgresql::Connection, Result};
use super::schema::tags;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub color: String,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn all(&self) -> Result<Vec<Item>>;
    fn by_id(&self, id: i64) -> Result<Item>;
    fn by_code(&self, cod: &str) -> Result<Item>;
    fn create(&self, code: &str, name: &str, color: &str) -> Result<()>;
    fn update(&self, id: i64, code: &str, name: &str, color: &str) -> Result<()>;
    fn destory(&self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self) -> Result<Vec<Item>> {
        let items = tags::dsl::tags.order(tags::dsl::name.asc()).load(self)?;
        Ok(items)
    }
    fn by_id(&self, id: i64) -> Result<Item> {
        let it = tags::dsl::tags.filter(tags::dsl::id.eq(id)).first(self)?;
        Ok(it)
    }
    fn by_code(&self, code: &str) -> Result<Item> {
        let it = tags::dsl::tags
            .filter(tags::dsl::code.eq(code))
            .first(self)?;
        Ok(it)
    }
    fn create(&self, code: &str, name: &str, color: &str) -> Result<()> {
        let now = Utc::now().naive_local();
        insert_into(tags::dsl::tags)
            .values((
                tags::dsl::code.eq(code),
                tags::dsl::name.eq(name),
                tags::dsl::color.eq(color),
                tags::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&self, id: i64, code: &str, name: &str, color: &str) -> Result<()> {
        let now = Utc::now().naive_local();
        update(tags::dsl::tags.filter(tags::dsl::id.eq(&id)))
            .set((
                tags::dsl::code.eq(code),
                tags::dsl::name.eq(name),
                tags::dsl::color.eq(color),
                tags::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn destory(&self, id: i64) -> Result<()> {
        delete(tags::dsl::tags.filter(tags::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
