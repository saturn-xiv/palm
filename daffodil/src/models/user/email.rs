use std::fmt;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::super::schema::email_users;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub real_name: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
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
    pub const GUEST_NAME: &str = "Guest";

    pub fn guest_email() -> String {
        format!("{}@local", Uuid::new_v4().simple())
    }
    pub fn guest_nickname() -> String {
        Uuid::new_v4().simple().to_string()
    }
}

pub trait Dao {
    fn create(
        &mut self,
        user: i32,
        real_name: &str,
        nickname: &str,
        email: &str,
        password: &str,
    ) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_nickname(&mut self, nickname: &str) -> Result<Item>;
    fn by_email(&mut self, email: &str) -> Result<Item>;
    fn total(&mut self) -> Result<i64>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn confirm(&mut self, id: i32) -> Result<()>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn set_real_name(&mut self, id: i32, real_name: &str) -> Result<()>;
    fn set_avatar(&mut self, id: i32, avatar: &str) -> Result<()>;
    fn set_password(&mut self, id: i32, password: &str) -> Result<()>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        user: i32,
        real_name: &str,
        nickname: &str,
        email: &str,
        password: &str,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let password = super::Item::password(password)?;
        let avatar = super::Item::gravatar(&email.to_string())?;
        insert_into(email_users::dsl::email_users)
            .values((
                email_users::dsl::user_id.eq(user),
                email_users::dsl::real_name.eq(real_name),
                email_users::dsl::nickname.eq(nickname),
                email_users::dsl::email.eq(email),
                email_users::dsl::avatar.eq(avatar),
                email_users::dsl::password.eq(password),
                email_users::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = email_users::dsl::email_users
            .filter(email_users::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_nickname(&mut self, nickname: &str) -> Result<Item> {
        let it = email_users::dsl::email_users
            .filter(email_users::dsl::nickname.eq(nickname))
            .first(self)?;
        Ok(it)
    }
    fn by_email(&mut self, email: &str) -> Result<Item> {
        let it = email_users::dsl::email_users
            .filter(email_users::dsl::email.eq(email))
            .first(self)?;
        Ok(it)
    }
    fn total(&mut self) -> Result<i64> {
        let it: i64 = email_users::dsl::email_users.count().get_result(self)?;
        Ok(it)
    }
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = email_users::dsl::email_users
            .order(email_users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn confirm(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::confirmed_at.eq(&now),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::deleted_at.eq(&Some(now)),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_real_name(&mut self, id: i32, real_name: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::real_name.eq(real_name),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_avatar(&mut self, id: i32, avatar: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::avatar.eq(avatar),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_password(&mut self, id: i32, password: &str) -> Result<()> {
        let password = super::Item::password(password)?;
        let now = Utc::now().naive_utc();
        let it = email_users::dsl::email_users.filter(email_users::dsl::id.eq(id));
        update(it)
            .set((
                email_users::dsl::password.eq(&password),
                email_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
