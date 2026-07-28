use std::any::type_name;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use hyacinth::schema::logs;
use strum::{Display as StrumDisplay, EnumString};

use super::super::{Result, orm::postgresql::Connection};

#[derive(Debug, PartialEq, EnumString, StrumDisplay)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub plugin: String,
    pub level: String,
    pub ip: String,
    pub message: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add<M: Into<String>, P>(
        &mut self,
        user: i64,
        level: Level,
        ip: &str,
        message: M,
    ) -> Result<()>;

    fn all(&mut self, user: i64, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_plugin<P>(&mut self) -> Result<Vec<Item>>;
    fn count(&mut self, user: i64) -> Result<i64>;
}

impl Dao for Connection {
    fn add<M: Into<String>, P>(
        &mut self,
        user: i64,
        level: Level,
        ip: &str,
        message: M,
    ) -> Result<()> {
        insert_into(logs::dsl::logs)
            .values((
                logs::dsl::user_id.eq(user),
                logs::dsl::ip.eq(ip),
                logs::dsl::plugin.eq(type_name::<P>()),
                logs::dsl::level.eq(&level.to_string()),
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
    fn by_plugin<P>(&mut self) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::plugin.eq(type_name::<P>()))
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
