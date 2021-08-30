use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use serde::Serialize;

use super::super::super::super::{orm::postgresql::Connection, Result};
use super::schema::{policies, resources};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub type_: String,
    pub code: String,
    pub name: String,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub const CODE_ALL: &'static str = "*";
    pub const TYPE_ALL: &'static str = "*";
}

pub trait Dao {
    fn all(&self) -> Result<Vec<Item>>;
    fn by_id(&self, id: i64) -> Result<Item>;
    fn by_code(&self, user: &str) -> Result<Item>;
    fn by_type_and_code(&self, type_: &str, code: &str) -> Result<Item>;
    fn create(&self, type_: &str, code: &str, name: &str) -> Result<()>;
    fn update(&self, id: i64, type_: &str, code: &str, name: &str) -> Result<()>;
    fn destory(&self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self) -> Result<Vec<Item>> {
        let items = resources::dsl::resources
            .order(resources::dsl::name.asc())
            .load(self)?;
        Ok(items)
    }
    fn by_id(&self, id: i64) -> Result<Item> {
        let it = resources::dsl::resources
            .filter(resources::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_type_and_code(&self, type_: &str, code: &str) -> Result<Item> {
        let it = resources::dsl::resources
            .filter(resources::dsl::type_.eq(type_))
            .filter(resources::dsl::code.eq(code))
            .first(self)?;
        Ok(it)
    }
    fn by_code(&self, code: &str) -> Result<Item> {
        let it = resources::dsl::resources
            .filter(resources::dsl::code.eq(code))
            .first(self)?;
        Ok(it)
    }
    fn create(&self, type_: &str, code: &str, name: &str) -> Result<()> {
        let now = Utc::now().naive_local();
        insert_into(resources::dsl::resources)
            .values((
                resources::dsl::type_.eq(type_),
                resources::dsl::code.eq(code),
                resources::dsl::name.eq(name),
                resources::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&self, id: i64, type_: &str, code: &str, name: &str) -> Result<()> {
        let now = Utc::now().naive_local();
        update(resources::dsl::resources.filter(resources::dsl::id.eq(&id)))
            .set((
                resources::dsl::type_.eq(type_),
                resources::dsl::code.eq(code),
                resources::dsl::name.eq(name),
                resources::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn destory(&self, id: i64) -> Result<()> {
        delete(policies::dsl::policies.filter(policies::dsl::resource_id.eq(id))).execute(self)?;
        delete(resources::dsl::resources.filter(resources::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
