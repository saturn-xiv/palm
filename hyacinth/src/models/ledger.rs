use std::string::ToString;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::bookkeeper_ledgers;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub uid: String,
    pub label: String,
    pub memo: String,
    pub profile: Vec<u8>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {}

pub trait Dao {
    fn create(&mut self, user: i32, label: &str, memo: &str) -> Result<String>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>>;
    fn set_details(&mut self, id: i32, label: &str, memo: &str) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn create(&mut self, user: i32, label: &str, memo: &str) -> Result<String> {
        let now = Utc::now().naive_utc();
        let uid = Uuid::new_v4().to_string();
        let profile = flexbuffers::to_vec(Profile::default())?;
        insert_into(bookkeeper_ledgers::dsl::bookkeeper_ledgers)
            .values((
                bookkeeper_ledgers::dsl::user_id.eq(user),
                bookkeeper_ledgers::dsl::uid.eq(&uid),
                bookkeeper_ledgers::dsl::label.eq(label),
                bookkeeper_ledgers::dsl::memo.eq(memo),
                bookkeeper_ledgers::dsl::profile.eq(&profile),
                bookkeeper_ledgers::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(uid)
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bookkeeper_ledgers::dsl::bookkeeper_ledgers
            .filter(bookkeeper_ledgers::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        let it = bookkeeper_ledgers::dsl::bookkeeper_ledgers
            .filter(bookkeeper_ledgers::dsl::uid.eq(uid))
            .first(self)?;
        Ok(it)
    }
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_ledgers::dsl::bookkeeper_ledgers
            .filter(bookkeeper_ledgers::dsl::user_id.eq(user))
            .order(bookkeeper_ledgers::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn set_details(&mut self, id: i32, label: &str, memo: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            bookkeeper_ledgers::dsl::bookkeeper_ledgers.filter(bookkeeper_ledgers::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_ledgers::dsl::label.eq(label),
                bookkeeper_ledgers::dsl::memo.eq(memo),
                bookkeeper_ledgers::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            bookkeeper_ledgers::dsl::bookkeeper_ledgers.filter(bookkeeper_ledgers::dsl::id.eq(id));
        update(it)
            .set((bookkeeper_ledgers::dsl::deleted_at.eq(&Some(now)),))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let it =
            bookkeeper_ledgers::dsl::bookkeeper_ledgers.filter(bookkeeper_ledgers::dsl::id.eq(id));
        update(it)
            .set((bookkeeper_ledgers::dsl::deleted_at.eq(&None::<NaiveDateTime>),))
            .execute(self)?;
        Ok(())
    }
}
