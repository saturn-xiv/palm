use std::string::ToString;

use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use hyper::StatusCode;
use juniper::GraphQLEnum;
use petunia::{orm::postgresql::Connection, HttpError, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};
use uuid::Uuid;

use super::super::schema::sessions;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub uid: String,
    pub provider_type: String,
    pub provider_id: i32,
    pub ip: String,
    pub expires_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl Item {
    pub fn available(&self) -> Result<()> {
        let now = Utc::now().naive_utc();
        if self.deleted_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::FORBIDDEN,
                Some("session is forbidden".to_string()),
            )));
        }
        if self.expires_at < now {
            return Err(Box::new(HttpError(
                StatusCode::GONE,
                Some("session is expired".to_string()),
            )));
        }
        Ok(())
    }
}

#[derive(EnumString, EnumDisplay, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Confirm,
    Unlock,
    #[strum(serialize = "reset-password")]
    #[serde(rename = "reset-password")]
    ResetPassword,
    #[strum(serialize = "sign-in")]
    #[serde(rename = "sign-in")]
    SignIn,
}

#[derive(
    GraphQLEnum, EnumString, EnumDisplay, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy,
)]
#[serde(rename_all = "camelCase")]
#[graphql(name = "UserProviderType")]
pub enum ProviderType {
    Email,
    Google,
    Facebook,
    WechatMiniProgram,
    WechatOauth2,
}

pub trait Dao {
    fn create(
        &mut self,
        user: i32,
        provider_type: ProviderType,
        provider_id: i32,
        ip: &str,
        ttl: Duration,
    ) -> Result<String>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn by_user_and_provider_type(
        &mut self,
        user: i32,
        provider_type: ProviderType,
    ) -> Result<Vec<Item>>;
    fn by_ip(&mut self, ip: &str) -> Result<Vec<Item>>;
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>>;
    fn index(&mut self) -> Result<Vec<Item>>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn clean(&mut self) -> Result<()>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        user: i32,
        provider_type: ProviderType,
        provider_id: i32,
        ip: &str,
        ttl: Duration,
    ) -> Result<String> {
        let uid = Uuid::new_v4().to_string();
        let expires_at = Utc::now().naive_utc() + ttl;
        let provider_type = provider_type.to_string();
        insert_into(sessions::dsl::sessions)
            .values((
                sessions::dsl::user_id.eq(user),
                sessions::dsl::provider_type.eq(provider_type),
                sessions::dsl::provider_id.eq(provider_id),
                sessions::dsl::ip.eq(ip),
                sessions::dsl::expires_at.eq(expires_at),
            ))
            .execute(self)?;
        Ok(uid)
    }
    fn by_user_and_provider_type(
        &mut self,
        user: i32,
        provider_type: ProviderType,
    ) -> Result<Vec<Item>> {
        let provider_type = provider_type.to_string();
        let items = sessions::dsl::sessions
            .filter(sessions::dsl::user_id.eq(user))
            .filter(sessions::dsl::provider_type.eq(provider_type))
            .load(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = sessions::dsl::sessions
            .filter(sessions::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        let it = sessions::dsl::sessions
            .filter(sessions::dsl::uid.eq(uid))
            .first(self)?;
        Ok(it)
    }
    fn by_ip(&mut self, ip: &str) -> Result<Vec<Item>> {
        let items = sessions::dsl::sessions
            .filter(sessions::dsl::ip.eq(ip))
            .load(self)?;
        Ok(items)
    }
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>> {
        let items = sessions::dsl::sessions
            .filter(sessions::dsl::user_id.eq(user))
            .load(self)?;
        Ok(items)
    }
    fn index(&mut self) -> Result<Vec<Item>> {
        let items = sessions::dsl::sessions
            .order(sessions::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = sessions::dsl::sessions.filter(sessions::dsl::id.eq(id));
        update(it)
            .set((sessions::dsl::deleted_at.eq(&Some(now)),))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let it = sessions::dsl::sessions.filter(sessions::dsl::id.eq(id));
        update(it)
            .set((sessions::dsl::deleted_at.eq(&None::<NaiveDateTime>),))
            .execute(self)?;
        Ok(())
    }
    fn clean(&mut self) -> Result<()> {
        let now = Utc::now().naive_utc();
        delete(sessions::dsl::sessions.filter(sessions::dsl::expires_at.lt(now))).execute(self)?;
        Ok(())
    }
}
