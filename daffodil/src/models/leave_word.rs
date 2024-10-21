use std::string::ToString;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Editor, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

use super::super::schema::leave_words;

#[derive(EnumDisplay, EnumString, Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    #[default]
    Pending,
    Closed,
}

#[derive(Queryable, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub lang: String,
    pub ip: String,
    pub body: String,
    pub body_editor: String,
    pub status: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn create(&mut self, lang: &str, ip: &str, body: &str, editor: Editor) -> Result<()>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count(&mut self) -> Result<i64>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn set_status(&mut self, id: i32, status: Status) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = leave_words::dsl::leave_words
            .filter(leave_words::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(&mut self, lang: &str, ip: &str, body: &str, editor: Editor) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(leave_words::dsl::leave_words)
            .values((
                leave_words::lang.eq(lang),
                leave_words::ip.eq(ip),
                leave_words::body.eq(body),
                leave_words::body_editor.eq(&editor.to_string()),
                leave_words::status.eq(&Status::Pending.to_string()),
                leave_words::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = leave_words::dsl::leave_words
            .order(leave_words::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn count(&mut self) -> Result<i64> {
        let it = leave_words::dsl::leave_words.count().first(self)?;
        Ok(it)
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = leave_words::dsl::leave_words.filter(leave_words::dsl::id.eq(id));
        update(it)
            .set((
                leave_words::dsl::deleted_at.eq(&Some(now)),
                leave_words::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = leave_words::dsl::leave_words.filter(leave_words::dsl::id.eq(id));
        update(it)
            .set((
                leave_words::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                leave_words::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_status(&mut self, id: i32, status: Status) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = leave_words::dsl::leave_words.filter(leave_words::dsl::id.eq(id));
        update(it)
            .set((
                leave_words::dsl::status.eq(&status.to_string()),
                leave_words::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
