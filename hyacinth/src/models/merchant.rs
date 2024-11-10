use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::schema::bookkeeper_merchants;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub label: String,
    pub memo: String,
    pub contact: Option<String>,
    pub addresses: Vec<u8>,
    pub phones: Vec<u8>,
    pub maps: Vec<u8>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn create(&mut self, ledger: i32, label: &str, memo: &str) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>>;
    fn set_details(&mut self, id: i32, label: &str, memo: &str) -> Result<()>;
    fn set_contact(&mut self, id: i32, contact: &str) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn create(&mut self, ledger: i32, label: &str, memo: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(bookkeeper_merchants::dsl::bookkeeper_merchants)
            .values((
                bookkeeper_merchants::dsl::ledger_id.eq(ledger),
                bookkeeper_merchants::dsl::label.eq(label),
                bookkeeper_merchants::dsl::memo.eq(memo),
                bookkeeper_merchants::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bookkeeper_merchants::dsl::bookkeeper_merchants
            .filter(bookkeeper_merchants::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_merchants::dsl::bookkeeper_merchants
            .filter(bookkeeper_merchants::dsl::ledger_id.eq(ledger))
            .order(bookkeeper_merchants::dsl::label.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn set_details(&mut self, id: i32, label: &str, memo: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bookkeeper_merchants::dsl::bookkeeper_merchants
            .filter(bookkeeper_merchants::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_merchants::dsl::label.eq(label),
                bookkeeper_merchants::dsl::memo.eq(memo),
                bookkeeper_merchants::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_contact(&mut self, id: i32, contact: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bookkeeper_merchants::dsl::bookkeeper_merchants
            .filter(bookkeeper_merchants::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_merchants::dsl::contact.eq(contact),
                bookkeeper_merchants::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bookkeeper_merchants::dsl::bookkeeper_merchants
            .filter(bookkeeper_merchants::dsl::id.eq(id));
        update(it)
            .set((bookkeeper_merchants::dsl::deleted_at.eq(&Some(now)),))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let it = bookkeeper_merchants::dsl::bookkeeper_merchants
            .filter(bookkeeper_merchants::dsl::id.eq(id));
        update(it)
            .set((bookkeeper_merchants::dsl::deleted_at.eq(&None::<NaiveDateTime>),))
            .execute(self)?;
        Ok(())
    }
}
