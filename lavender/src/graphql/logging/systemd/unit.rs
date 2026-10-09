use chrono::NaiveDateTime;
use juniper::GraphQLObject;
use portal::{
    Jwt, Result,
    cache::redis::StandaloneConnection as Cache,
    graphql::Session,
    graphql::{Page, Pagination},
    opensearch::{Client as Search, timestamp},
    orm::postgresql::Connection as Db,
    rbac::Rbac,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::super::super::can;

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "LavenderSystemdUnitLog")]
pub struct Item {
    pub host: String,
    pub unit: String,
    pub priority: i32,
    pub message: String,
    #[serde(with = "timestamp")]
    pub created_at: NaiveDateTime,
}

impl Item {
    pub fn queries_by_unit(unit: &str) -> (Value, Value) {
        (
            json!({
                "query": {
                    "term": {
                        "unit": unit
                    }
                }
            }),
            json!({
                "query": {
                    "term": {
                        "unit": unit
                    }
                },
                "sort": [
                    {
                        "created_at": {
                            "order": "desc"
                        }
                    }
                ]
            }),
        )
    }
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "LavenderIndexSystemdUnitLog")]
pub struct Index {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}

impl Index {
    pub async fn by_unit<R: Rbac, J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        search: &Search,
        (unit, page): (&str, &Page),
    ) -> Result<Self> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        can(rbac, current_user.id()).await?;

        let (count, query) = Item::queries_by_unit(unit);
        let (items, pagination) = search.pagination::<Item>(count, query, page).await?;

        Ok(Self { items, pagination })
    }
}
