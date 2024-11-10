use std::ops::DerefMut;

use juniper::GraphQLObject;
use petunia::{orm::postgresql::Pool as DbPool, Result};

use super::super::models::currency::{Dao as CurrencyDao, Item as Currency};

#[derive(GraphQLObject)]
#[graphql(name = "Currency")]
pub struct Item {
    pub id: i32,
    pub code: String,
    pub number: String,
    pub name: String,
    pub country: String,
    pub units: i32,
}

impl From<Currency> for Item {
    fn from(it: Currency) -> Self {
        Self {
            id: it.id,
            code: it.code.clone(),
            name: it.name.clone(),
            number: it.number.clone(),
            country: it.country.clone(),
            units: it.units,
        }
    }
}

impl Item {
    pub fn all(db: &DbPool) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let mut items = Vec::new();
        for it in CurrencyDao::all(db)? {
            items.push(it.into());
        }
        Ok(items)
    }
}
