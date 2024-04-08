use casbin::Enforcer;
use juniper::GraphQLObject;
use palm::{
    cache::redis::{nodes as cluster_nodes, ClusterConnection as Cache},
    jwt::Jwt,
    session::Session,
    Error,
};
use tokio::sync::Mutex;

use super::super::super::super::{orm::postgresql::Connection as Db, services::CurrentUserAdapter};

#[derive(GraphQLObject)]
#[graphql(name = "RedisStatus")]
pub struct Status {
    pub nodes: Vec<String>,
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
        let nodes = cluster_nodes(ch)?;
        Ok(Self { nodes })
    }
}
