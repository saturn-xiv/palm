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

use super::super::can;

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "LavenderHttpResponseItem")]
pub struct Item {
    pub from: String,
    pub url: String,
    pub status_code: Option<i32>,
    pub content_type: Option<String>,
    pub body: String,
    pub elapsed: i32,
    #[serde(with = "timestamp")]
    pub created_at: NaiveDateTime,
}

impl Item {
    pub fn queries_by_url(url: &str) -> (Value, Value) {
        (
            json!({
                "query": {
                    "term": {
                        "url": url
                    }
                }
            }),
            json!({
                "query": {
                    "term": {
                        "url": url
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
#[graphql(name = "LavenderIndexHttpResponse")]
pub struct Index {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}

impl Index {
    pub async fn by_url<R: Rbac, J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        search: &Search,
        (url, page): (&str, &Page),
    ) -> Result<Self> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        can(rbac, current_user.id()).await?;

        let (count, query) = Item::queries_by_url(url);
        let (items, pagination) = search.pagination::<Item>(count, query, page).await?;

        Ok(Self { items, pagination })
    }
}
