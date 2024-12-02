use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use juniper::GraphQLObject;
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::super::schema::postal_addresses;

#[derive(GraphQLObject, Queryable, Serialize, Deserialize, Clone)]
#[graphql(name = "PostalAddress")]
pub struct Item {
    pub id: i32,
    pub unit: Option<String>,
    pub building: Option<String>,
    pub street: String,
    pub city: String,
    pub province: String,
    pub country: String,
    pub zip_code: String,
    pub passcode: Option<String>,
    pub google_map: Option<String>,
    pub a_map: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn create(
        &mut self,
        line1: (Option<&str>, Option<&str>, &str),
        line2: (&str, &str, &str),
        zip_code: &str,
        passcode: Option<&str>,
        map: (Option<&str>, Option<&str>),
    ) -> Result<i32>;
    fn update(
        &mut self,
        id: i32,
        line1: (Option<&str>, Option<&str>, &str),
        line2: (&str, &str, &str),
        zip_code: &str,
        passcode: Option<&str>,
        map: (Option<&str>, Option<&str>),
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
        (unit, building, street): (Option<&str>, Option<&str>, &str),
        (city, province, country): (&str, &str, &str),
        zip_code: &str,
        passcode: Option<&str>,
        (google_map, a_map): (Option<&str>, Option<&str>),
    ) -> Result<i32> {
        let now = Utc::now().naive_utc();
        let id = insert_into(postal_addresses::dsl::postal_addresses)
            .values((
                postal_addresses::dsl::unit.eq(unit),
                postal_addresses::dsl::building.eq(building),
                postal_addresses::dsl::street.eq(street),
                postal_addresses::dsl::city.eq(city),
                postal_addresses::dsl::province.eq(province),
                postal_addresses::dsl::country.eq(country),
                postal_addresses::dsl::zip_code.eq(zip_code),
                postal_addresses::dsl::passcode.eq(passcode),
                postal_addresses::dsl::google_map.eq(google_map),
                postal_addresses::dsl::a_map.eq(a_map),
                postal_addresses::dsl::updated_at.eq(&now),
            ))
            .returning(postal_addresses::dsl::id)
            .get_result(self)?;
        Ok(id)
    }
    fn update(
        &mut self,
        id: i32,
        (unit, building, street): (Option<&str>, Option<&str>, &str),
        (city, province, country): (&str, &str, &str),
        zip_code: &str,
        passcode: Option<&str>,
        (google_map, a_map): (Option<&str>, Option<&str>),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = postal_addresses::dsl::postal_addresses.filter(postal_addresses::dsl::id.eq(id));
        update(it)
            .set((
                postal_addresses::dsl::unit.eq(unit),
                postal_addresses::dsl::building.eq(building),
                postal_addresses::dsl::street.eq(street),
                postal_addresses::dsl::city.eq(city),
                postal_addresses::dsl::province.eq(province),
                postal_addresses::dsl::country.eq(country),
                postal_addresses::dsl::zip_code.eq(zip_code),
                postal_addresses::dsl::passcode.eq(passcode),
                postal_addresses::dsl::google_map.eq(google_map),
                postal_addresses::dsl::a_map.eq(a_map),
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
