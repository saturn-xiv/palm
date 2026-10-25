use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use super::super::super::{
    Result, models::setting::Dao as SettingDao, orm::postgresql::Connection as Db,
};

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "Oauth")]
pub struct Item {
    pub by_email: bool,
    pub by_sms: bool,
    pub by_google_oauth2: bool,
    pub by_facebook_oauth2: bool,
    pub by_wechat_oauth2: bool,
    pub by_wechat_mini_program: bool,
}
impl Default for Item {
    fn default() -> Self {
        Self {
            by_email: true,
            by_sms: true,
            by_google_oauth2: true,
            by_facebook_oauth2: true,
            by_wechat_oauth2: true,
            by_wechat_mini_program: true,
        }
    }
}
impl Item {
    pub const KEY: &str = "site.oauth";

    pub fn new(db: &mut Db) -> Result<Self> {
        let it = SettingDao::get(db, &Self::KEY.to_string(), None)?;
        let it = flexbuffers::from_slice(&it.value)?;
        Ok(it)
    }
}
