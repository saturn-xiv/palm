use std::fmt;
use std::string::ToString;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use hyper::StatusCode;
use palm::{openssl::gravatar, HttpError, Password, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::super::{orm::postgresql::Connection, schema::email_users};

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub real_name: String,
    pub nickname: String,
    pub email: String,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
    pub avatar: String,
    pub confirmed_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl fmt::Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}<{}>", self.real_name, self.email)
    }
}

impl Item {
    pub const GUEST_NAME: &'static str = "Guest";
    pub const GUEST_LANG: &'static str = "en-US";
    pub const GUEST_TIMEZONE: &'static str = "UTC";
    pub const NIL: &'static str = "nil";

    pub fn guest_email() -> String {
        format!("{}@local", Uuid::new_v4().simple())
    }
    pub fn guest_nickname() -> String {
        Uuid::new_v4().simple().to_string()
    }

    pub fn available(&self) -> Result<()> {
        if self.deleted_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::GONE,
                Some("user is deleted".to_string()),
            )));
        }
        if self.confirmed_at.is_none() {
            return Err(Box::new(HttpError(
                StatusCode::PRECONDITION_REQUIRED,
                Some("user isn't confirmed".to_string()),
            )));
        }
        Ok(())
    }
    pub fn auth<P: Password>(&self, enc: &P, password: &str) -> Result<()> {
        if enc.verify(&self.password, password.as_bytes(), &self.salt) {
            return Ok(());
        }

        Err(Box::new(HttpError(
            StatusCode::UNAUTHORIZED,
            Some("bad password".to_string()),
        )))
    }

    const SALT_SIZE: usize = 16;
}

pub trait Dao {
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_email(&mut self, email: &str) -> Result<Item>;
    fn by_nickname(&mut self, nickname: &str) -> Result<Item>;
    fn set_real_name(&mut self, id: i64, real_name: &str) -> Result<()>;
    fn set_avatar(&mut self, id: i64, avatar: &str) -> Result<()>;
    fn create<P: Password>(
        &mut self,
        enc: &P,
        real_name: &str,
        nickname: &str,
        email: &str,
        password: &str,
    ) -> Result<()>;
    fn confirm(&mut self, id: i64) -> Result<()>;
    fn count(&mut self) -> Result<i64>;
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn password<P: Password>(&mut self, enc: &P, id: i64, password: &str) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = email_users::dsl::email_users
            .filter(email_users::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn by_email(&mut self, email: &str) -> Result<Item> {
        let it = email_users::dsl::email_users
            .filter(email_users::dsl::email.eq(&email))
            .first(self)?;
        Ok(it)
    }

    fn by_nickname(&mut self, nickname: &str) -> Result<Item> {
        let it = email_users::dsl::email_users
            .filter(email_users::dsl::nickname.eq(nickname))
            .first(self)?;
        Ok(it)
    }

    fn create<P: Password>(
        &mut self,
        enc: &P,
        real_name: &str,
        nickname: &str,
        email: &str,
        password: &str,
    ) -> Result<()> {
        let (password, salt) = enc.compute(password.as_bytes(), Item::SALT_SIZE)?;
        insert_into(email_users::dsl::email_users)
            .values((
                email_users::dsl::real_name.eq(real_name),
                email_users::dsl::nickname.eq(nickname),
                email_users::dsl::email.eq(email),
                email_users::dsl::password.eq(&password),
                email_users::dsl::salt.eq(&salt),
                email_users::dsl::avatar.eq(&gravatar(&email)?),
                email_users::dsl::updated_at.eq(&Utc::now().naive_utc()),
            ))
            .execute(self)?;
        Ok(())
    }

    fn set_real_name(&mut self, id: i64, real_name: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(email_users::dsl::email_users.filter(email_users::dsl::id.eq(id)))
            .set((
                email_users::dsl::real_name.eq(real_name),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn set_avatar(&mut self, id: i64, avatar: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(email_users::dsl::email_users.filter(email_users::dsl::id.eq(id)))
            .set((
                email_users::dsl::avatar.eq(avatar),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn confirm(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::confirmed_at.eq(&Some(now)),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn count(&mut self) -> Result<i64> {
        let cnt: i64 = email_users::dsl::email_users.count().get_result(self)?;
        Ok(cnt)
    }

    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = email_users::dsl::email_users
            .order(email_users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }

    fn password<P: Password>(&mut self, enc: &P, id: i64, password: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let (password, salt) = enc.compute(password.as_bytes(), Item::SALT_SIZE)?;
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::password.eq(&password),
                email_users::dsl::salt.eq(&salt),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
