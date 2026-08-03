use chrono::NaiveDateTime;
use juniper::GraphQLObject;

use super::super::{
    Result,
    models::currency::{Dao as CurrencyDao, Item as CurrencyItem},
    orm::postgresql::Connection as Db,
};

pub fn index(db: &mut Db) -> Result<Vec<Item>> {
    let items = CurrencyDao::index(db)?;

    let mut reply = Vec::new();
    for it in items {
        reply.push(it.into());
    }
    Ok(reply)
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "Currency")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub country: String,
    pub number: i32,
    pub units: Option<i32>,
    pub fund: Option<bool>,
    pub created_at: NaiveDateTime,
}

impl From<CurrencyItem> for Item {
    fn from(it: CurrencyItem) -> Self {
        Self {
            id: it.id as i32,
            name: it.name.clone(),
            code: it.code.clone(),
            country: it.country.clone(),
            number: it.number,
            units: it.units,
            fund: it.fund,
            created_at: it.created_at,
        }
    }
}
