use serde::{Deserialize, Serialize};

use super::super::super::{
    Result, models::setting::Dao as SettingDao, orm::postgresql::Connection as Db,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Keywords(pub Vec<String>);

impl Keywords {
    pub const KEY: &str = "site.keywords";
    pub fn new(db: &mut Db) -> Result<Self> {
        let it = SettingDao::get(db, &Self::KEY.to_string(), None)?;
        Ok(flexbuffers::from_slice(&it.value)?)
    }
}
