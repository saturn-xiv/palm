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
    pub status_code: i32,
    pub content_type: String,
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
        let total = search
            .count_document_by_query::<Item>(json!({
                "query": {
                    "term": {
                        "url": url
                    }
                }
            }))
            .await? as i64;

        let items = search
            .search_document::<Item>(json!({
                "from": page.offset(total),
                "size": page.size(),
                "query": {
                    "term": {
                        "url": url
                    }
                }
            }))
            .await?;
        Ok(Self {
            items: items
                .hits
                .hits
                .into_iter()
                .map(|x| x._source)
                .collect::<_>(),
            pagination: Pagination::new(page, total),
        })
    }
}
