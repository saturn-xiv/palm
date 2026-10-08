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
#[graphql(name = "LavenderKubernatesLogItem")]
pub struct Item {
    pub node: String,
    pub pod: String,
    pub container: String,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "LavenderIndexKubernatesLog")]
pub struct Index {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}
impl Index {
    pub async fn by_namespace<R: Rbac, J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        search: &Search,
        (namespace, page): (&str, &Page),
    ) -> Result<Self> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        can(rbac, current_user.id()).await?;

        let (items, pagination) = search
            .pagination::<Item>(
                json!({
                    "query": {
                        "term": {
                            "namespace": namespace
                        }
                    }
                }),
                json!({
                    "query": {
                        "term": {
                            "namespace": namespace
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
