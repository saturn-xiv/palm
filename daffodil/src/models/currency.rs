use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use petunia::{orm::postgresql::Connection, Result};
use serde::Serialize;

use super::super::schema::currencies;

#[derive(Queryable, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub code: String,
    pub number: String,
    pub name: String,
    pub country: String,
    pub units: i32,
    pub created_at: NaiveDateTime,
}
pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn all(&mut self) -> Result<Vec<Item>>;
    fn create(
        &mut self,
        code: &str,
        number: &str,
        name: &str,
        country: &str,
        units: i32,
    ) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = currencies::dsl::currencies
            .filter(currencies::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        code: &str,
        number: &str,
        name: &str,
        country: &str,
        units: i32,
    ) -> Result<()> {
        insert_into(currencies::dsl::currencies)
            .values((
                currencies::code.eq(code),
                currencies::number.eq(number),
                currencies::name.eq(name),
                currencies::country.eq(country),
                currencies::units.eq(units),
            ))
            .execute(self)?;
        Ok(())
    }
    fn all(&mut self) -> Result<Vec<Item>> {
        let items = currencies::dsl::currencies
            .order(currencies::dsl::country.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
}
