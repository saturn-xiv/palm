pub mod email;
pub mod google_oauth2;
pub mod wechat;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordVerifier, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{NaiveDateTime, Utc};
use chrono_tz::Tz;
use data_encoding::{BASE64_NOPAD, HEXLOWER};
use diesel::{insert_into, prelude::*, update};
use hyper::StatusCode;
use juniper::GraphQLObject;
use language_tags::LanguageTag;
use petunia::{orm::postgresql::Connection, HttpError, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::super::schema::{email_users, users};
use super::session::ProviderType;

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
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
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
    fn create(&mut self, uid: &str, lang: &str, timezone: &str) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn total(&mut self) -> Result<i64>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn lock(&mut self, id: i32) -> Result<()>;
    fn unlock(&mut self, id: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn sign_in(&mut self, id: i32, ip: &str) -> Result<()>;
    fn sign_out(&mut self, id: i32) -> Result<()>;
    fn set_lang(&mut self, id: i32, lang: &str) -> Result<()>;
    fn set_timezone(&mut self, id: i32, timezone: &str) -> Result<()>;
}

impl Dao for Connection {
    fn create(&mut self, uid: &str, lang: &str, timezone: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(users::dsl::users)
            .values((
                users::dsl::uid.eq(uid),
                users::dsl::lang.eq(lang),
                users::dsl::timezone.eq(timezone),
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
    fn sign_out(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let (current_sign_in_at, current_sign_in_ip) = users::dsl::users
            .select((
                users::dsl::current_sign_in_at,
                users::dsl::current_sign_in_ip,
            ))
            .filter(users::dsl::id.eq(id))
            .first::<(Option<NaiveDateTime>, Option<String>)>(self)?;
        update(users::dsl::users.filter(users::dsl::id.eq(id)))
            .set((
                users::dsl::current_sign_in_at.eq(&None::<NaiveDateTime>),
                users::dsl::current_sign_in_ip.eq(&None::<String>),
                users::dsl::last_sign_in_at.eq(&current_sign_in_at),
                users::dsl::last_sign_in_ip.eq(&current_sign_in_ip),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_lang(&mut self, id: i32, lang: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((users::dsl::lang.eq(lang), users::dsl::updated_at.eq(&now)))
            .execute(self)?;
        Ok(())
    }
    fn set_timezone(&mut self, id: i32, timezone: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::timezone.eq(timezone),
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

    pub fn password(plain: &str) -> Result<String> {
        let plain = Self::from_mingle_password(plain)?;
        let salt = SaltString::generate(&mut OsRng);

        let cipher = Argon2::default()
            .hash_password(&plain, &salt)
            .map_err(|x| {
                Box::new(HttpError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(x.to_string()),
                ))
            })?
            .to_string();
        Ok(cipher)
    }

    // Math.random().toString(36).slice(2)
    fn from_mingle_password(plain: &str) -> Result<Vec<u8>> {
        let salt_size = 11;
        if plain.len() <= salt_size {
            return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
        }
        let plain = {
            let it = &plain[salt_size..];
            BASE64_NOPAD.decode(it.as_bytes())?
        };
        Ok(plain)
    }

    pub fn verify(plain: &str, cipher: &str) -> Result<()> {
        let plain = Self::from_mingle_password(plain)?;
        let hash = PasswordHash::new(cipher).map_err(|x| {
            Box::new(HttpError(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(x.to_string()),
            ))
        })?;

        Argon2::default()
            .verify_password(&plain, &hash)
            .map_err(|x| {
                Box::new(HttpError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(x.to_string()),
                ))
            })?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "UserSelectOption")]
pub struct SelectOption {
    pub id: i32,
    pub provider_id: i32,
    pub provider_type: ProviderType,
    pub label: String,
    pub lang: String,
    pub timezone: String,
}

impl SelectOption {
    pub fn new(db: &mut Connection, id: i32) -> Result<Self> {
        let (lang, timezone) = users::dsl::users
            .select((users::dsl::lang, users::dsl::timezone))
            .filter(users::dsl::id.eq(id))
            .filter(users::dsl::deleted_at.is_null())
            .filter(users::dsl::locked_at.is_null())
            .order(users::dsl::updated_at.desc())
            .first::<(String, String)>(db)?;

        Self::by_id(db, id, lang, timezone)
    }
    pub fn all(db: &mut Connection) -> Result<Vec<Self>> {
        let mut items = Vec::new();
        for (id, lang, timezone) in users::dsl::users
            .select((users::dsl::id, users::dsl::lang, users::dsl::timezone))
            .filter(users::dsl::deleted_at.is_null())
            .filter(users::dsl::locked_at.is_null())
            .order(users::dsl::updated_at.desc())
            .load::<(i32, String, String)>(db)?
        {
            if let Ok(it) = Self::by_id(db, id, lang, timezone) {
                items.push(it);
            }
        }

        Ok(items)
    }

    fn by_id(db: &mut Connection, id: i32, lang: String, timezone: String) -> Result<Self> {
        if let Ok((eu_id, real_name, email)) = email_users::dsl::email_users
            .select((
                email_users::dsl::id,
                email_users::dsl::real_name,
                email_users::dsl::email,
            ))
            .filter(email_users::dsl::user_id.eq(id))
            .filter(email_users::dsl::confirmed_at.is_not_null())
            .filter(email_users::dsl::deleted_at.is_null())
            .first::<(i32, String, String)>(db)
        {
            return Ok(Self {
                id,
                provider_id: eu_id,
                provider_type: ProviderType::Email,
                label: format!("{real_name}<{email}>"),
                lang,
                timezone,
            });
        }

        Err(Box::new(HttpError(StatusCode::NOT_FOUND, None)))
    }
}
