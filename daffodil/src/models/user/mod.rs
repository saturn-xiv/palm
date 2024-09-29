pub mod email;
pub mod google_oauth2;
pub mod wechat;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{NaiveDateTime, Utc};
use chrono_tz::Tz;
use data_encoding::HEXLOWER;
use diesel::{insert_into, prelude::*, update};
use hyper::StatusCode;
use language_tags::LanguageTag;
use petunia::{orm::postgresql::Connection, HttpError, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use strum::{Display as EnumDisplay, EnumString};

use super::super::schema::users;

#[derive(EnumString, EnumDisplay, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    SignIn,
    ResetPassword,
    Unlock,
    Confirm,
    Other(String),
}

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub uid: String,
    pub lang: String,
    pub timezone: String,
    pub sign_in_count: i32,
    pub current_sign_in_at: Option<NaiveDateTime>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_at: Option<NaiveDateTime>,
    pub last_sign_in_ip: Option<String>,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn guest_lang() -> Result<LanguageTag> {
        let it = LanguageTag::parse("en-US")?;
        Ok(it)
    }
    pub fn guest_timezone() -> Tz {
        Tz::UTC
    }
}

pub trait Dao {
    fn create(&mut self, uid: &str, lang: &LanguageTag, timezone: Tz) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn total(&mut self) -> Result<i64>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn lock(&mut self, id: i32) -> Result<()>;
    fn unlock(&mut self, id: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn sign_in(&mut self, id: i32, ip: &str) -> Result<()>;
    fn set_lang(&mut self, id: i32, lang: &LanguageTag) -> Result<()>;
    fn set_timezone(&mut self, id: i32, timezone: Tz) -> Result<()>;
}

impl Dao for Connection {
    fn create(&mut self, uid: &str, lang: &LanguageTag, timezone: Tz) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(users::dsl::users)
            .values((
                users::dsl::uid.eq(uid),
                users::dsl::lang.eq(&lang.to_string()),
                users::dsl::timezone.eq(&timezone.to_string()),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = users::dsl::users
            .filter(users::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        let it = users::dsl::users
            .filter(users::dsl::uid.eq(uid))
            .first(self)?;
        Ok(it)
    }
    fn total(&mut self) -> Result<i64> {
        let it: i64 = users::dsl::users.count().get_result(self)?;
        Ok(it)
    }
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = users::dsl::users
            .order(users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn lock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::locked_at.eq(&Some(now)),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::locked_at.eq(&None::<NaiveDateTime>),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::deleted_at.eq(&Some(now)),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn sign_in(&mut self, id: i32, ip: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let (current_sign_in_at, current_sign_in_ip, sign_in_count) = users::dsl::users
            .select((
                users::dsl::current_sign_in_at,
                users::dsl::current_sign_in_ip,
                users::dsl::sign_in_count,
            ))
            .filter(users::dsl::id.eq(id))
            .first::<(Option<NaiveDateTime>, Option<String>, i32)>(self)?;
        update(users::dsl::users.filter(users::dsl::id.eq(id)))
            .set((
                users::dsl::current_sign_in_at.eq(&now),
                users::dsl::current_sign_in_ip.eq(&Some(ip)),
                users::dsl::last_sign_in_at.eq(&current_sign_in_at),
                users::dsl::last_sign_in_ip.eq(&current_sign_in_ip),
                users::dsl::sign_in_count.eq(&(sign_in_count + 1)),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_lang(&mut self, id: i32, lang: &LanguageTag) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::lang.eq(&lang.to_string()),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_timezone(&mut self, id: i32, timezone: Tz) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::timezone.eq(&timezone.to_string()),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}

impl Item {
    // https://en.gravatar.com/site/implement/hash/
    pub fn gravatar<S: AsRef<str>>(email: &S) -> Result<String> {
        let hash = Sha256::digest(email.as_ref().to_lowercase().trim().as_bytes());
        let it = format!(
            "https://www.gravatar.com/avatar/{}.png",
            HEXLOWER.encode(&hash)
        );
        Ok(it)
    }

    pub fn password(plain: &str) -> Result<Vec<u8>> {
        let plain = plain.as_bytes();
        let mut salt = Vec::new();
        {
            let it = SaltString::generate(&mut OsRng);
            it.decode_b64(&mut salt).map_err(|x| {
                Box::new(HttpError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(x.to_string()),
                ))
            })?;
        };

        let mut cipher = [0u8; 32];
        Argon2::default()
            .hash_password_into(plain, &salt, &mut cipher)
            .map_err(|x| {
                Box::new(HttpError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(x.to_string()),
                ))
            })?;
        Ok(cipher.to_vec())
    }

    pub fn verify(plain: &str, cipher: &str) -> Result<()> {
        let hash = PasswordHash::new(cipher).map_err(|x| {
            Box::new(HttpError(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(x.to_string()),
            ))
        })?;
        let plain = plain.as_bytes();
        Argon2::default()
            .verify_password(plain, &hash)
            .map_err(|x| {
                Box::new(HttpError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(x.to_string()),
                ))
            })?;
        Ok(())
    }
}
