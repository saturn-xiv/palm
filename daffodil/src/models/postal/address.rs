use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use juniper::{GraphQLInputObject, GraphQLObject};
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

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(name = "PostalAddressForm")]
pub struct Form {
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
}

pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn create(&mut self, form: &Form) -> Result<i32>;
    fn update(&mut self, id: i32, form: &Form) -> Result<()>;
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
    fn create(&mut self, form: &Form) -> Result<i32> {
        let now = Utc::now().naive_utc();
        let id = insert_into(postal_addresses::dsl::postal_addresses)
            .values((
                postal_addresses::dsl::unit.eq(&form.unit),
                postal_addresses::dsl::building.eq(&form.building),
                postal_addresses::dsl::street.eq(&form.street),
                postal_addresses::dsl::city.eq(&form.city),
                postal_addresses::dsl::province.eq(&form.province),
                postal_addresses::dsl::country.eq(&form.country),
                postal_addresses::dsl::zip_code.eq(&form.zip_code),
                postal_addresses::dsl::passcode.eq(&form.passcode),
                postal_addresses::dsl::google_map.eq(&form.google_map),
                postal_addresses::dsl::a_map.eq(&form.a_map),
                postal_addresses::dsl::updated_at.eq(&now),
            ))
            .returning(postal_addresses::dsl::id)
            .get_result(self)?;
        Ok(id)
    }
    fn update(&mut self, id: i32, form: &Form) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = postal_addresses::dsl::postal_addresses.filter(postal_addresses::dsl::id.eq(id));
        update(it)
            .set((
                postal_addresses::dsl::unit.eq(&form.unit),
                postal_addresses::dsl::building.eq(&form.building),
                postal_addresses::dsl::street.eq(&form.street),
                postal_addresses::dsl::city.eq(&form.city),
                postal_addresses::dsl::province.eq(&form.province),
                postal_addresses::dsl::country.eq(&form.country),
                postal_addresses::dsl::zip_code.eq(&form.zip_code),
                postal_addresses::dsl::passcode.eq(&form.passcode),
                postal_addresses::dsl::google_map.eq(&form.google_map),
                postal_addresses::dsl::a_map.eq(&form.a_map),
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
