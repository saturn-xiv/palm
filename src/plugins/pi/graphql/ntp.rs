use std::fs::File;
use std::io::prelude::*;
use std::path::{Component, Path};

use chrono::{DateTime, Local};
use chrono_tz::Tz;
use juniper::GraphQLObject;
use serde::{self, Deserialize, Serialize};
use validator::Validate;

use super::super::super::super::{
    crypto::Aes, graphql::Session, jwt::Jwt, ntp::Response, orm::Connection as Db,
    settings::Dao as SettingDao, Result,
};

#[derive(Serialize, Deserialize, Validate, GraphQLObject, Debug)]
#[graphql(name = "OpenVpn")]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = 1))]
    pub timezone: String,
    #[validate(length(min = 1))]
    pub server: String,
    #[validate(range(min = 5))]
    pub heartbeat: i32,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            timezone: Tz::UTC.name().to_string(),
            server: "0.us.pool.ntp.org".to_string(),
            heartbeat: 60 * 60 * 24,
        }
    }
}

impl Form {
    pub fn timesyncd(&self) -> String {
        format!(
            r#"
[Time]
NTP={server}
FallbackNTP=0.pool.ntp.org 1.pool.ntp.org 2.pool.ntp.org 3.pool.ntp.org
        "#,
            server = self.server
        )
    }
    pub fn crontab(&self) -> String {
        format!(
            r#"
#!/bin/sh
ntpdate {server}
        "#,
            server = self.server
        )
    }

    pub fn test(&self) -> Result<(DateTime<Local>, Tz)> {
        let tz = self.timezone.parse::<Tz>()?;
        let now: DateTime<Local> = Response::fetch(&self.server, None)?.into();
        Ok((now, tz))
    }
}

impl Form {
    pub const KEY: &'static str = "ntp.client";
    pub fn new(ss: &Session, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<Self> {
        ss.current_user(db, jwt)?;
        let it: Self = SettingDao::get(db, aes, Self::KEY).unwrap_or_default();
        Ok(it)
    }
    pub fn save(&self, ss: &Session, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<()> {
        self.validate()?;
        ss.current_user(db, jwt)?;
        debug!("save ntp server {:?}", self);
        let mut fd = File::create(
            Path::new(&Component::RootDir)
                .join("etc")
                .join("systemd")
                .join("timesyncd.conf"),
        )?;
        write!(&mut fd, "{}", self.timesyncd())?;
        SettingDao::set(db, aes, Self::KEY, self, true)?;
        Ok(())
    }
}
