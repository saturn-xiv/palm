use camelia::orm::postgresql::Connection;
use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use palm::Result;
use serde::{Deserialize, Serialize};

use super::super::schema::daffodil_books;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: String,
    pub cover_id: Option<i64>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_user(&mut self, user: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn create(
        &mut self,
        user: i64,
        name: &str,
        description: &str,
        cover: Option<i64>,
    ) -> Result<()>;
    fn update(&mut self, id: i64, name: &str, description: &str, cover: Option<i64>) -> Result<()>;
    fn enable(&mut self, id: i64, on: bool) -> Result<()>;
}

impl Dao for Connection {
    fn by_user(&mut self, user: i64) -> Result<Vec<Item>> {
        let items = daffodil_books::dsl::daffodil_books
            .filter(daffodil_books::dsl::user_id.eq(user))
            .order(daffodil_books::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = daffodil_books::dsl::daffodil_books
            .filter(daffodil_books::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i64,
        name: &str,
        description: &str,
        cover: Option<i64>,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(daffodil_books::dsl::daffodil_books)
            .values((
                daffodil_books::dsl::user_id.eq(user),
                daffodil_books::dsl::name.eq(name),
                daffodil_books::dsl::description.eq(description),
                daffodil_books::dsl::cover_id.eq(cover),
                daffodil_books::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i64, name: &str, description: &str, cover: Option<i64>) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = daffodil_books::dsl::daffodil_books.filter(daffodil_books::dsl::id.eq(id));
        update(it)
            .set((
                daffodil_books::dsl::name.eq(name),
                daffodil_books::dsl::description.eq(description),
                daffodil_books::dsl::cover_id.eq(cover),
                daffodil_books::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i64, on: bool) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = daffodil_books::dsl::daffodil_books.filter(daffodil_books::dsl::id.eq(id));
        update(it)
            .set((
                daffodil_books::dsl::deleted_at.eq(&if on { None } else { Some(now) }),
                daffodil_books::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
