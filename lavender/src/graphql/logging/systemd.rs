use chrono::NaiveDateTime;
use juniper::GraphQLObject;
use portal::{
    Jwt, Result,
    cache::redis::StandaloneConnection as Cache,
    graphql::Session,
    graphql::{Page, Pagination},
    opensearch::Client as Search,
    orm::postgresql::Connection as Db,
    rbac::Rbac,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::super::can;

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "LavenderSystemdLogItem")]
pub struct Item {
    pub host: String,
    pub unit: String,
    pub priority: i32,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "LavenderIndexSystemdLog")]
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

        let (items, pagination) = search
            .pagination::<Item>(
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
                page,
            )
            .await?;

        Ok(Self { items, pagination })
    }
}
