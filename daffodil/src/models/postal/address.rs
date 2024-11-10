use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::super::schema::postal_addresses;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip_code: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn create(
        &mut self,
        street: &str,
        city: &str,
        state: &str,
        country: &str,
        zip_code: &str,
    ) -> Result<()>;
    fn update(
        &mut self,
        id: i32,
        street: &str,
        city: &str,
        state: &str,
        country: &str,
        zip_code: &str,
    ) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = postal_addresses::dsl::postal_addresses
            .filter(postal_addresses::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        street: &str,
        city: &str,
        state: &str,
        country: &str,
        zip_code: &str,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(postal_addresses::dsl::postal_addresses)
            .values((
                postal_addresses::dsl::street.eq(street),
                postal_addresses::dsl::city.eq(city),
                postal_addresses::dsl::state.eq(state),
                postal_addresses::dsl::country.eq(country),
                postal_addresses::dsl::zip_code.eq(zip_code),
                postal_addresses::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(
        &mut self,
        id: i32,
        street: &str,
        city: &str,
        state: &str,
        country: &str,
        zip_code: &str,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = postal_addresses::dsl::postal_addresses.filter(postal_addresses::dsl::id.eq(id));
        update(it)
            .set((
                postal_addresses::dsl::street.eq(street),
                postal_addresses::dsl::city.eq(city),
                postal_addresses::dsl::state.eq(state),
                postal_addresses::dsl::country.eq(country),
                postal_addresses::dsl::zip_code.eq(zip_code),
                postal_addresses::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = postal_addresses::dsl::postal_addresses.filter(postal_addresses::dsl::id.eq(id));
        update(it)
            .set((
                postal_addresses::dsl::deleted_at.eq(&Some(now)),
                postal_addresses::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = postal_addresses::dsl::postal_addresses.filter(postal_addresses::dsl::id.eq(id));
        update(it)
            .set((
                postal_addresses::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                postal_addresses::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
