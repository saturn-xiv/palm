use casbin::Enforcer;
use diesel::prelude::*;
use hibiscus::{cache::redis::ClusterConnection as Cache, jwt::Jwt, session::Session, Error};
use juniper::GraphQLObject;
use tokio::sync::Mutex;

use super::super::super::super::{
    orm::{postgresql::Connection as Db, Dao as VersionDao},
    schema::schema_migrations,
    services::CurrentUserAdapter,
};

#[derive(GraphQLObject)]
#[graphql(name = "PostgreSqlStatus")]
pub struct Status {
    pub migrations: Vec<String>,
    pub version: String,
}

impl Status {
    pub async fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
    ) -> Result<Self, Error> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        user.is_administrator(enf).await?;

        let migrations = schema_migrations::dsl::schema_migrations
            .select(schema_migrations::dsl::version)
            .load(db)?;
        let version = VersionDao::version(db)?;

        Ok(Self {
            migrations,
            version,
        })
    }
}
