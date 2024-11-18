use chrono::NaiveDateTime;
use juniper::GraphQLObject;
use petunia::{
    orm::{postgresql::Connection as Db, Dao as VersionDao},
    Result,
};

#[derive(GraphQLObject)]
#[graphql(name = "PostgreSqlStatus")]
pub struct Status {
    pub timestamp: NaiveDateTime,
    pub version: String,
}

impl Status {
    pub fn new(db: &mut Db) -> Result<Self> {
        let it = Self {
            timestamp: VersionDao::timestamp(db)?,
            version: VersionDao::version(db)?,
        };
        Ok(it)
    }
}
