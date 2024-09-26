use std::any::type_name;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

use super::super::schema::logs;

#[derive(EnumDisplay, EnumString, Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Level {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub plugin: String,
    pub ip: String,
    pub level: String,
    pub resource_type: String,
    pub resource_id: Option<i32>,
    pub message: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn create<S: Into<String>, T>(
        &mut self,
        user: i32,
        plugin: &str,
        level: Level,
        ip: &str,
        resource_id: Option<i32>,
        message: S,
    ) -> Result<()>;
    fn create_<S: Into<String>>(
        &mut self,
        user: i32,
        plugin: &str,
        level: Level,
        ip: &str,
        resource: (&str, Option<i32>),
        message: S,
    ) -> Result<()>;

    fn by_resource<T>(&mut self, resource_id: Option<i32>) -> Result<Vec<Item>>;
    fn by_resource_type<T>(&mut self) -> Result<Vec<Item>>;
    fn by_resource_(&mut self, resource_type: &str, resource_id: Option<i32>) -> Result<Vec<Item>>;
    fn by_resource_type_(&mut self, resource_type: &str) -> Result<Vec<Item>>;
    fn index_by_user(&mut self, user: i32, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count_by_user(&mut self, user: i32) -> Result<i64>;
}

impl Dao for Connection {
    fn create<S: Into<String>, T>(
        &mut self,
        user: i32,
        plugin: &str,
        level: Level,
        ip: &str,
        resource_id: Option<i32>,
        message: S,
    ) -> Result<()> {
        self.create_(
            user,
            plugin,
            level,
            ip,
            (type_name::<T>(), resource_id),
            message,
        )
    }
    fn create_<S: Into<String>>(
        &mut self,
        user: i32,
        plugin: &str,
        level: Level,
        ip: &str,
        (resource_type, resource_id): (&str, Option<i32>),
        message: S,
    ) -> Result<()> {
        insert_into(logs::dsl::logs)
            .values((
                logs::dsl::user_id.eq(user),
                logs::dsl::plugin.eq(plugin),
                logs::dsl::ip.eq(ip),
                logs::dsl::level.eq(&level.to_string()),
                logs::dsl::resource_type.eq(resource_type),
                logs::dsl::resource_id.eq(resource_id),
                logs::dsl::message.eq(&message.into()),
            ))
            .execute(self)?;
        Ok(())
    }

    fn index_by_user(&mut self, user: i32, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .order(logs::dsl::created_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_resource<T>(&mut self, resource_id: Option<i32>) -> Result<Vec<Item>> {
        self.by_resource_(type_name::<T>(), resource_id)
    }
    fn by_resource_type<T>(&mut self) -> Result<Vec<Item>> {
        self.by_resource_type_(type_name::<T>())
    }
    fn by_resource_(&mut self, resource_type: &str, resource_id: Option<i32>) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::resource_type.eq(resource_type))
            .filter(logs::dsl::resource_id.eq(resource_id))
            .order(logs::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_resource_type_(&mut self, resource_type: &str) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::resource_type.eq(resource_type))
            .order(logs::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn count_by_user(&mut self, user: i32) -> Result<i64> {
        let it = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .count()
            .first(self)?;
        Ok(it)
    }
}
