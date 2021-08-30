use std::fmt;
use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use super::super::super::super::{
    orm::postgresql::Connection, Error, HttpError, MediaType, Result,
};
use super::schema::notifications;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Level {
    Warning,
    Info,
    Error,
}

impl Level {
    const WARNING: &'static str = "warning";
    const INFO: &'static str = "info";
    const ERROR: &'static str = "error";
}

impl fmt::Display for Level {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Info => write!(fmt, "{}", Self::INFO),
            Self::Warning => write!(fmt, "{}", Self::WARNING),
            Self::Error => write!(fmt, "{}", Self::ERROR),
        }
    }
}

impl FromStr for Level {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            Self::ERROR => Ok(Self::Error),
            Self::INFO => Ok(Self::Info),
            Self::WARNING => Ok(Self::Warning),
            _ => Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("unknown notification level {:?}", s)),
            ))),
        }
    }
}

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub url: String,
    pub body: String,
    pub meta_type: String,
    pub level: String,
    pub read: bool,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn all(&self, user: i64) -> Result<Vec<Item>>;
    fn by_id(&self, id: i64) -> Result<Item>;
    fn unread(&self, user: i64) -> Result<Vec<Item>>;
    fn create(&self, user: i64, body: &str, media_type: &MediaType, level: &Level) -> Result<()>;
    fn set_read(&self, id: i64) -> Result<()>;
    fn destory(&self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self, user: i64) -> Result<Vec<Item>> {
        let items = notifications::dsl::notifications
            .filter(notifications::dsl::user_id.eq(user))
            .order(notifications::dsl::created_at.desc())
            .load(self)?;
        Ok(items)
    }
    fn by_id(&self, id: i64) -> Result<Item> {
        let it = notifications::dsl::notifications
            .filter(notifications::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn unread(&self, user: i64) -> Result<Vec<Item>> {
        let items = notifications::dsl::notifications
            .filter(notifications::dsl::user_id.eq(user))
            .filter(notifications::dsl::read.eq(false))
            .order(notifications::dsl::created_at.desc())
            .load(self)?;
        Ok(items)
    }
    fn create(&self, user: i64, body: &str, media_type: &MediaType, level: &Level) -> Result<()> {
        let now = Utc::now().naive_local();
        insert_into(notifications::dsl::notifications)
            .values((
                notifications::dsl::user_id.eq(user),
                notifications::dsl::body.eq(body),
                notifications::dsl::media_type.eq(&media_type.to_string()),
                notifications::dsl::level.eq(&level.to_string()),
                notifications::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_read(&self, id: i64) -> Result<()> {
        update(notifications::dsl::notifications.filter(notifications::dsl::id.eq(&id)))
            .set(notifications::dsl::read.eq(true))
            .execute(self)?;
        Ok(())
    }
    fn destory(&self, id: i64) -> Result<()> {
        delete(notifications::dsl::notifications.filter(notifications::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
}
