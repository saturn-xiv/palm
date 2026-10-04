use chrono::NaiveDateTime;
use juniper::GraphQLObject;

use super::super::super::super::{
    Result,
    orm::{Dao as HeartbeatDao, Heartbeat, postgresql::Connection as Db},
};

#[derive(Debug, Clone, Default, GraphQLObject)]
#[graphql(name = "PostgreSqlStatus")]
pub struct Item {
    pub version: String,
    pub created_at: NaiveDateTime,
}

impl Item {
    pub fn new(db: &mut Db) -> Result<Self> {
        let it = HeartbeatDao::heartbeat(db)?;
        Ok(it.into())
    }
}

impl From<Heartbeat> for Item {
    fn from(it: Heartbeat) -> Self {
        Self {
            created_at: it.created_at,
            version: it.version,
        }
    }
}
