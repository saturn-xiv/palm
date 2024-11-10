use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::super::schema::postal_recipients;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub country_code: String,
    pub phone: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn create(&mut self, name: &str, country_code: &str, phone: &str) -> Result<()>;
    fn update(&mut self, id: i32, name: &str, country_code: &str, phone: &str) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = postal_recipients::dsl::postal_recipients
            .filter(postal_recipients::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(&mut self, name: &str, country_code: &str, phone: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(postal_recipients::dsl::postal_recipients)
            .values((
                postal_recipients::dsl::name.eq(name),
                postal_recipients::dsl::country_code.eq(country_code),
                postal_recipients::dsl::phone.eq(phone),
                postal_recipients::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i32, name: &str, country_code: &str, phone: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            postal_recipients::dsl::postal_recipients.filter(postal_recipients::dsl::id.eq(id));
        update(it)
            .set((
                postal_recipients::dsl::name.eq(name),
                postal_recipients::dsl::country_code.eq(country_code),
                postal_recipients::dsl::phone.eq(phone),
                postal_recipients::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            postal_recipients::dsl::postal_recipients.filter(postal_recipients::dsl::id.eq(id));
        update(it)
            .set((
                postal_recipients::dsl::deleted_at.eq(&Some(now)),
                postal_recipients::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            postal_recipients::dsl::postal_recipients.filter(postal_recipients::dsl::id.eq(id));
        update(it)
            .set((
                postal_recipients::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                postal_recipients::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
