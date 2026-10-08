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
#[graphql(name = "LavenderHttpResponseItem")]
pub struct Item {
    pub from: String,
    pub url: String,
    pub status_code: Optional<i32>,
    pub content_type: Optional<String>,
    pub body: String,
    pub elapsed: i32,
    pub created_at: NaiveDateTime,
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
        let (items, pagination) = search
            .pagination::<Item>(
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
                page,
            )
            .await?;

        Ok(Self { items, pagination })
    }
}
