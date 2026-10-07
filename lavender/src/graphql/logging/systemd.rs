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
        _search: &Search,
        (_unit, _page): (&str, &Page),
    ) -> Result<Vec<Self>> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        can(rbac, current_user.id()).await?;
        // TODO
        todo!()
    }
}
