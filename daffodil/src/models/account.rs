use camelia::orm::postgresql::Connection;
use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use palm::{daffodil::v1, Result};
use serde::{Deserialize, Serialize};

use super::super::schema::daffodil_accounts;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub book_id: i64,
    pub name: String,
    pub currency: String,
    pub type_: i32,
    pub description: String,
    pub cover_id: Option<i64>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_user(&mut self, user: i64) -> Result<Vec<Item>>;
    fn by_book(&mut self, book: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn create(
        &mut self,
        user: i64,
        book: i64,
        name: &str,
        type_: v1::account_index_response::item::Type,
        description: &str,
        cover: Option<i64>,
    ) -> Result<()>;
    fn update(&mut self, id: i64, name: &str, description: &str, cover: Option<i64>) -> Result<()>;
    fn enable(&mut self, id: i64, on: bool) -> Result<()>;
}

impl Dao for Connection {
    fn by_user(&mut self, user: i64) -> Result<Vec<Item>> {
        let items = daffodil_accounts::dsl::daffodil_accounts
            .filter(daffodil_accounts::dsl::user_id.eq(user))
            .order(daffodil_accounts::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_book(&mut self, book: i64) -> Result<Vec<Item>> {
        let items = daffodil_accounts::dsl::daffodil_accounts
            .filter(daffodil_accounts::dsl::book_id.eq(book))
            .order(daffodil_accounts::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = daffodil_accounts::dsl::daffodil_accounts
            .filter(daffodil_accounts::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i64,
        book: i64,
        name: &str,
        type_: v1::account_index_response::item::Type,
        description: &str,
        cover: Option<i64>,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(daffodil_accounts::dsl::daffodil_accounts)
            .values((
                daffodil_accounts::dsl::user_id.eq(user),
                daffodil_accounts::dsl::book_id.eq(book),
                daffodil_accounts::dsl::name.eq(name),
                daffodil_accounts::dsl::description.eq(description),
                daffodil_accounts::dsl::type_.eq(type_ as i32),
                daffodil_accounts::dsl::cover_id.eq(cover),
                daffodil_accounts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i64, name: &str, description: &str, cover: Option<i64>) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            daffodil_accounts::dsl::daffodil_accounts.filter(daffodil_accounts::dsl::id.eq(id));
        update(it)
            .set((
                daffodil_accounts::dsl::name.eq(name),
                daffodil_accounts::dsl::description.eq(description),
                daffodil_accounts::dsl::cover_id.eq(cover),
                daffodil_accounts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i64, on: bool) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            daffodil_accounts::dsl::daffodil_accounts.filter(daffodil_accounts::dsl::id.eq(id));
        update(it)
            .set((
                daffodil_accounts::dsl::deleted_at.eq(&if on { None } else { Some(now) }),
                daffodil_accounts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
