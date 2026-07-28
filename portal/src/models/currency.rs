use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use hyacinth::schema::currencies;

use super::super::{Result, orm::postgresql::Connection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub country: String,
    pub number: i32,
    pub units: Option<i32>,
    pub fund: Option<bool>,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn count(&mut self) -> Result<i64>;
    fn all(&mut self) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn create(
        &mut self,
        name: &str,
        code: &str,
        country: &str,
        number: i32,
        units: Option<i32>,
        fund: Option<bool>,
    ) -> Result<()>;
}

impl Dao for Connection {
    fn count(&mut self) -> Result<i64> {
        let it: i64 = currencies::dsl::currencies.count().get_result(self)?;
        Ok(it)
    }

    fn all(&mut self) -> Result<Vec<Item>> {
        let items = currencies::dsl::currencies
            .order(currencies::dsl::name.asc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = currencies::dsl::currencies
            .filter(currencies::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }

    fn create(
        &mut self,
        name: &str,
        code: &str,
        country: &str,
        number: i32,
        units: Option<i32>,
        fund: Option<bool>,
    ) -> Result<()> {
        insert_into(currencies::dsl::currencies)
            .values((
                currencies::dsl::name.eq(name),
                currencies::dsl::code.eq(code),
                currencies::dsl::country.eq(country),
                currencies::dsl::number.eq(number),
                currencies::dsl::units.eq(units),
                currencies::dsl::fund.eq(fund),
            ))
            .execute(self)?;
        Ok(())
    }
}
