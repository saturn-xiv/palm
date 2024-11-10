use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::schema::bookkeeper_categories;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub parent_id: Option<i32>,
    pub label: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn create(&mut self, ledger: i32, parent: Option<i32>, label: &str) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>>;
    fn by_parent(&mut self, parent: i32) -> Result<Vec<Item>>;
    fn set_label(&mut self, id: i32, label: &str) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn create(&mut self, ledger: i32, parent: Option<i32>, label: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(bookkeeper_categories::dsl::bookkeeper_categories)
            .values((
                bookkeeper_categories::dsl::ledger_id.eq(ledger),
                bookkeeper_categories::dsl::parent_id.eq(parent),
                bookkeeper_categories::dsl::label.eq(label),
                bookkeeper_categories::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bookkeeper_categories::dsl::bookkeeper_categories
            .filter(bookkeeper_categories::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_categories::dsl::bookkeeper_categories
            .filter(bookkeeper_categories::dsl::ledger_id.eq(ledger))
            .order(bookkeeper_categories::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_parent(&mut self, parent: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_categories::dsl::bookkeeper_categories
            .filter(bookkeeper_categories::dsl::parent_id.eq(parent))
            .order(bookkeeper_categories::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn set_label(&mut self, id: i32, label: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bookkeeper_categories::dsl::bookkeeper_categories
            .filter(bookkeeper_categories::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_categories::dsl::label.eq(label),
                bookkeeper_categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = bookkeeper_categories::dsl::bookkeeper_categories
            .filter(bookkeeper_categories::dsl::id.eq(id));
        update(it)
            .set((bookkeeper_categories::dsl::deleted_at.eq(&Some(now)),))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let it = bookkeeper_categories::dsl::bookkeeper_categories
            .filter(bookkeeper_categories::dsl::id.eq(id));
        update(it)
            .set((bookkeeper_categories::dsl::deleted_at.eq(&None::<NaiveDateTime>),))
            .execute(self)?;
        Ok(())
    }
}
