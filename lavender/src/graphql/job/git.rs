use chrono::NaiveDateTime;
use juniper::GraphQLObject;
use portal::{
    Jwt, Result, cache::redis::StandaloneConnection as Cache, graphql::Session,
    orm::postgresql::Connection as Db, rbac::Rbac,
};

use super::super::super::Config;
use super::super::ROLE as OPERATOR;

#[derive(Debug, GraphQLObject)]
#[graphql(name = "LavenderGitCommit")]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub created_at: NaiveDateTime,
}

impl Commit {
    pub async fn index<R: Rbac, J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        _config: &Config,
        (_url, _branch): (&str, &str),
    ) -> Result<Vec<Self>> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        rbac.has_role(current_user.id(), OPERATOR).await?;
        // TODO
        todo!()
    }
}
