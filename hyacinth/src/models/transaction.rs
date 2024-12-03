use std::string::ToString;

use chrono::{NaiveDateTime, Utc};
use chrono_tz::Tz;
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::bookkeeper_transactions;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub uid: String,
    pub ledger_id: i32,
    pub memo: String,
    pub traded_at: NaiveDateTime,
    pub timezone: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn create(
        &mut self,
        ledger: i32,
        memo: &str,
        traded_at: NaiveDateTime,
        timezone: Tz,
    ) -> Result<String>;
    fn update(&mut self, id: i32, memo: &str, traded_at: NaiveDateTime, timezone: Tz)
        -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        ledger: i32,
        memo: &str,
        traded_at: NaiveDateTime,
        timezone: Tz,
    ) -> Result<String> {
        let uid = Uuid::new_v4().to_string();
        let now = Utc::now().naive_utc();
        insert_into(bookkeeper_transactions::dsl::bookkeeper_transactions)
            .values((
                bookkeeper_transactions::dsl::uid.eq(&uid),
                bookkeeper_transactions::dsl::ledger_id.eq(ledger),
                bookkeeper_transactions::dsl::traded_at.eq(traded_at),
                bookkeeper_transactions::dsl::memo.eq(memo),
                bookkeeper_transactions::dsl::timezone.eq(&timezone.to_string()),
                bookkeeper_transactions::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(uid)
    }
    fn update(
        &mut self,
        id: i32,
        memo: &str,
        traded_at: NaiveDateTime,
        timezone: Tz,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bookkeeper_transactions::dsl::bookkeeper_transactions
            .filter(bookkeeper_transactions::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_transactions::dsl::traded_at.eq(traded_at),
                bookkeeper_transactions::dsl::memo.eq(memo),
                bookkeeper_transactions::dsl::timezone.eq(&timezone.to_string()),
                bookkeeper_transactions::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bookkeeper_transactions::dsl::bookkeeper_transactions
            .filter(bookkeeper_transactions::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        let it = bookkeeper_transactions::dsl::bookkeeper_transactions
            .filter(bookkeeper_transactions::dsl::uid.eq(uid))
            .first(self)?;
        Ok(it)
    }
    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_transactions::dsl::bookkeeper_transactions
            .filter(bookkeeper_transactions::dsl::ledger_id.eq(ledger))
            .order(bookkeeper_transactions::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bookkeeper_transactions::dsl::bookkeeper_transactions
            .filter(bookkeeper_transactions::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_transactions::dsl::deleted_at.eq(&Some(now)),
                bookkeeper_transactions::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bookkeeper_transactions::dsl::bookkeeper_transactions
            .filter(bookkeeper_transactions::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_transactions::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                bookkeeper_transactions::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
}
