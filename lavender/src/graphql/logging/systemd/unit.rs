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

use super::super::super::can;

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "LavenderSystemdUnitLog")]
pub struct Item {
    pub host: String,
    pub name: String,
    pub priority: i32,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "LavenderIndexSystemdUnitLog")]
pub struct Index {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}

impl Index {
    pub async fn by_name<R: Rbac, J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        search: &Search,
        (name, page): (&str, &Page),
    ) -> Result<Self> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        can(rbac, current_user.id()).await?;

        let (items, pagination) = search
            .pagination::<Item>(
                json!({
                    "query": {
                        "term": {
                            "name": name
                        }
                    }
                }),
                json!({
                    "query": {
                        "term": {
                            "name": name
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
