use std::fs::{remove_file, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Component, Path};

use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::super::super::super::{
    crypto::Aes, graphql::Session, jwt::Jwt, orm::Connection as Db, settings::Dao as SettingDao,
    Result,
};

#[derive(Serialize, Deserialize, Validate, GraphQLObject, Default)]
#[graphql(name = "OpenVpn")]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub enable: bool,
    #[validate(length(min = 1))]
    pub body: String,
}

impl Form {
    pub const KEY: &'static str = "openvpn.client";
    pub fn new(ss: &Session, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<Self> {
        ss.current_user(db, jwt)?;
        let it: Self = SettingDao::get(db, aes, Self::KEY).unwrap_or_default();
        Ok(it)
    }
    pub fn save(&self, ss: &Session, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<()> {
        self.validate()?;
        ss.current_user(db, jwt)?;
        SettingDao::set(db, aes, Self::KEY, self, true)?;
        let file = Path::new(&Component::RootDir)
            .join("etc")
            .join("openvpn")
            .join("client.conf");
        if self.enable {
            info!("generate file {}", file.display());
            let mut fd = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(file)?;
            fd.write_all(self.body.as_bytes())?;
        } else if file.exists() {
            info!("delete file {}", file.display());
            remove_file(file)?;
        }
        Ok(())
    }
}
