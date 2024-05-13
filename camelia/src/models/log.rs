use std::any::type_name;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use palm::{camelia::v1, Result};

use super::super::{orm::postgresql::Connection, schema::logs};

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub plugin: String,
    pub level: i32,
    pub ip: String,
    pub resource_type: String,
    pub resource_id: Option<i64>,
    pub message: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add<S: Into<String>, T>(
        &mut self,
        user: i64,
        plugin: &str,
        level: v1::user_logs_response::item::Level,
        ip: &str,
        resource_id: Option<i64>,
        message: S,
    ) -> Result<()>;

    fn all(&mut self, user: i64, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_resource<T>(&mut self, resource_id: Option<i64>) -> Result<Vec<Item>>;
    fn count(&mut self, user: i64) -> Result<i64>;
}

impl Dao for Connection {
    fn add<S: Into<String>, T>(
        &mut self,
        user: i64,
        plugin: &str,
        level: v1::user_logs_response::item::Level,
        ip: &str,
        resource_id: Option<i64>,
        message: S,
    ) -> Result<()> {
        insert_into(logs::dsl::logs)
            .values((
                logs::dsl::user_id.eq(user),
                logs::dsl::ip.eq(ip),
                logs::dsl::plugin.eq(plugin),
                logs::dsl::level.eq(level as i32),
                logs::dsl::resource_type.eq(type_name::<T>()),
                logs::dsl::resource_id.eq(resource_id),
                logs::dsl::message.eq(&message.into()),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&mut self, user: i64, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .order(logs::dsl::created_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_resource<T>(&mut self, resource_id: Option<i64>) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::resource_type.eq(type_name::<T>()))
            .filter(logs::dsl::resource_id.eq(resource_id))
            .order(logs::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn count(&mut self, user: i64) -> Result<i64> {
        let it = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .count()
            .first(self)?;
        Ok(it)
    }
}
