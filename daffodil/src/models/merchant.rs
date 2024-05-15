use camelia::orm::postgresql::Connection;
use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use palm::Result;
use serde::{Deserialize, Serialize};

use super::super::schema::daffodil_merchants;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub book_id: i64,
    pub name: String,
    pub address: Option<String>,
    pub contact: Option<String>,
    pub description: Option<String>,
    pub cover_id: Option<i64>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_book(&mut self, book: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn create(
        &mut self,
        user: i64,
        book: i64,
        name: &str,
        details: (Option<&str>, Option<&str>, Option<&str>),
        cover: Option<i64>,
    ) -> Result<()>;
    fn update(
        &mut self,
        id: i64,
        name: &str,
        address: Option<&str>,
        contact: Option<&str>,
        description: Option<&str>,
        cover: Option<i64>,
    ) -> Result<()>;
    fn enable(&mut self, id: i64, on: bool) -> Result<()>;
}

impl Dao for Connection {
    fn by_book(&mut self, book: i64) -> Result<Vec<Item>> {
        let items = daffodil_merchants::dsl::daffodil_merchants
            .filter(daffodil_merchants::dsl::book_id.eq(book))
            .order(daffodil_merchants::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = daffodil_merchants::dsl::daffodil_merchants
            .filter(daffodil_merchants::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i64,
        book: i64,
        name: &str,
        (address, contact, description): (Option<&str>, Option<&str>, Option<&str>),
        cover: Option<i64>,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(daffodil_merchants::dsl::daffodil_merchants)
            .values((
                daffodil_merchants::dsl::user_id.eq(user),
                daffodil_merchants::dsl::book_id.eq(book),
                daffodil_merchants::dsl::name.eq(name),
                daffodil_merchants::dsl::address.eq(address),
                daffodil_merchants::dsl::contact.eq(contact),
                daffodil_merchants::dsl::description.eq(description),
                daffodil_merchants::dsl::cover_id.eq(cover),
                daffodil_merchants::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(
        &mut self,
        id: i64,
        name: &str,
        address: Option<&str>,
        contact: Option<&str>,
        description: Option<&str>,
        cover: Option<i64>,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            daffodil_merchants::dsl::daffodil_merchants.filter(daffodil_merchants::dsl::id.eq(id));
        update(it)
            .set((
                daffodil_merchants::dsl::name.eq(name),
                daffodil_merchants::dsl::address.eq(address),
                daffodil_merchants::dsl::contact.eq(contact),
                daffodil_merchants::dsl::description.eq(description),
                daffodil_merchants::dsl::cover_id.eq(cover),
                daffodil_merchants::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i64, on: bool) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            daffodil_merchants::dsl::daffodil_merchants.filter(daffodil_merchants::dsl::id.eq(id));
        update(it)
            .set((
                daffodil_merchants::dsl::deleted_at.eq(&if on { None } else { Some(now) }),
                daffodil_merchants::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
