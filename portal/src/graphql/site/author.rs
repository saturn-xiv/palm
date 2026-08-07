use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use super::super::super::{
    Result, models::setting::Dao as SettingDao, orm::postgresql::Connection as Db,
};

#[derive(Debug, Default, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "SiteAuthor")]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl Author {
    pub const KEY: &str = "site.author";
    pub fn new(db: &mut Db) -> Result<Self> {
        let it = SettingDao::get(db, &Self::KEY.to_string(), None)?;
        Ok(flexbuffers::from_slice(&it.value)?)
    }
}
