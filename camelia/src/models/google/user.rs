use chrono::{NaiveDateTime, Utc};
use chrono_tz::Tz;
use diesel::{insert_into, prelude::*, update};
use hibiscus::google::oauth::openid::{
    AuthorizationCode as GoogleAuthorizationCode, IdToken as GoogleOpenIdToken,
};
use language_tags::LanguageTag;
use palm::{random::uuid, Result};
use serde::{Deserialize, Serialize};

use super::super::super::{orm::postgresql::Connection, schema::google_users};
use super::super::user::{Dao as UserDao, Item as User};

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub email: Option<String>,
    pub email_verified: bool,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub sub: String,
    pub code: Vec<u8>,
    pub token: String,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn all(&mut self) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_token(&mut self, token: &GoogleOpenIdToken) -> Result<Item>;
    fn sign_in(
        &mut self,
        user_id: Option<i64>,
        code: &GoogleAuthorizationCode,
        token: &GoogleOpenIdToken,
        ip: &str,
        lang: &LanguageTag,
        timezone: Tz,
    ) -> Result<User>;
    fn set_profile(&mut self, user: i64, token: &GoogleOpenIdToken) -> Result<()>;
    fn count_by_user(&mut self, user: i64) -> Result<i64>;
}

impl Dao for Connection {
    fn all(&mut self) -> Result<Vec<Item>> {
        let items = google_users::dsl::google_users
            .order(google_users::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = google_users::dsl::google_users
            .filter(google_users::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn set_profile(&mut self, user: i64, token: &GoogleOpenIdToken) -> Result<()> {
        let user = Dao::by_id(self, user)?;
        let now = Utc::now().naive_utc();
        if user.name != token.name {
            if let Some(ref name) = token.name {
                update(google_users::dsl::google_users.filter(google_users::dsl::id.eq(user.id)))
                    .set((
                        google_users::dsl::name.eq(&name),
                        google_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
        }
        if user.email != token.email {
            if let Some(ref email) = token.email {
                update(google_users::dsl::google_users.filter(google_users::dsl::id.eq(user.id)))
                    .set((
                        google_users::dsl::email.eq(&email),
                        google_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
        }
        if user.email_verified != token.email_verified {
            update(google_users::dsl::google_users.filter(google_users::dsl::id.eq(user.id)))
                .set((
                    google_users::dsl::email_verified.eq(token.email_verified),
                    google_users::dsl::updated_at.eq(&now),
                ))
                .execute(self)?;
        }
        if user.picture != token.picture {
            if let Some(ref avatar) = token.picture {
                update(google_users::dsl::google_users.filter(google_users::dsl::id.eq(user.id)))
                    .set((
                        google_users::dsl::picture.eq(&avatar),
                        google_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
        }
        Ok(())
    }

    fn by_token(&mut self, token: &GoogleOpenIdToken) -> Result<Item> {
        let it = google_users::dsl::google_users
            .filter(google_users::dsl::sub.eq(&token.sub))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn sign_in(
        &mut self,
        user_id: Option<i64>,
        code: &GoogleAuthorizationCode,
        token: &GoogleOpenIdToken,
        ip: &str,
        lang: &LanguageTag,
        timezone: Tz,
    ) -> Result<User> {
        let code_v = flexbuffers::to_vec(code)?;
        let token_v = serde_json::to_string(token)?;
        let now = Utc::now().naive_utc();

        let user = match google_users::dsl::google_users
            .filter(google_users::dsl::sub.eq(&token.sub))
            .first::<Item>(self)
        {
            Ok(it) => {
                update(google_users::dsl::google_users.filter(google_users::dsl::id.eq(it.id)))
                    .set((
                        google_users::dsl::code.eq(&code_v),
                        google_users::dsl::token.eq(&token_v),
                        google_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
                Dao::set_profile(self, it.user_id, token)?;
                UserDao::by_id(self, it.user_id)?
            }
            Err(_) => {
                let user = match user_id {
                    Some(id) => {
                        Dao::set_profile(self, id, token)?;
                        UserDao::by_id(self, id)?
                    }
                    None => {
                        let uid = uuid();
                        UserDao::create(self, &uid, lang, timezone)?;
                        UserDao::by_uid(self, &uid)?
                    }
                };
                insert_into(google_users::dsl::google_users)
                    .values((
                        google_users::dsl::user_id.eq(user.id),
                        google_users::dsl::sub.eq(&token.sub),
                        google_users::dsl::code.eq(&code_v),
                        google_users::dsl::token.eq(&token_v),
                        google_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
                user
            }
        };

        UserDao::sign_in(self, user.id, ip)?;
        Ok(user)
    }

    fn count_by_user(&mut self, user: i64) -> Result<i64> {
        let cnt: i64 = google_users::dsl::google_users
            .filter(google_users::dsl::user_id.eq(user))
            .count()
            .get_result(self)?;
        Ok(cnt)
    }
}
