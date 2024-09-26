use std::fmt;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::super::schema::wechat_mini_program_users;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub union_id: String,
    pub app_id: String,
    pub open_id: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl fmt::Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}://{}", self.app_id, self.open_id)
    }
}

pub trait Dao {
    fn create(
        &mut self,
        user: i32,
        wechat_id: (&str, &str, &str),
        info: (Option<&str>, Option<&str>),
    ) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_open_id(&mut self, app_id: &str, open_id: &str) -> Result<Item>;
    fn by_union_id(&mut self, union_id: &str) -> Result<Item>;
    fn total(&mut self) -> Result<i64>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn update(&mut self, id: i32, info: (Option<&str>, Option<&str>)) -> Result<()>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        user: i32,
        (union_id, app_id, open_id): (&str, &str, &str),
        (nickname, avatar_url): (Option<&str>, Option<&str>),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(wechat_mini_program_users::dsl::wechat_mini_program_users)
            .values((
                wechat_mini_program_users::dsl::user_id.eq(user),
                wechat_mini_program_users::dsl::union_id.eq(union_id),
                wechat_mini_program_users::dsl::app_id.eq(app_id),
                wechat_mini_program_users::dsl::open_id.eq(open_id),
                wechat_mini_program_users::dsl::nickname.eq(nickname),
                wechat_mini_program_users::dsl::avatar_url.eq(avatar_url),
                wechat_mini_program_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_open_id(&mut self, app_id: &str, open_id: &str) -> Result<Item> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::app_id.eq(app_id))
            .filter(wechat_mini_program_users::dsl::open_id.eq(open_id))
            .first(self)?;
        Ok(it)
    }
    fn by_union_id(&mut self, union_id: &str) -> Result<Item> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::union_id.eq(union_id))
            .first(self)?;
        Ok(it)
    }
    fn total(&mut self) -> Result<i64> {
        let it: i64 = wechat_mini_program_users::dsl::wechat_mini_program_users
            .count()
            .get_result(self)?;
        Ok(it)
    }
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = wechat_mini_program_users::dsl::wechat_mini_program_users
            .order(wechat_mini_program_users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_mini_program_users::dsl::deleted_at.eq(&Some(now)),
                wechat_mini_program_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_mini_program_users::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                wechat_mini_program_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(
        &mut self,
        id: i32,
        (nickname, avatar_url): (Option<&str>, Option<&str>),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_mini_program_users::dsl::nickname.eq(nickname),
                wechat_mini_program_users::dsl::avatar_url.eq(avatar_url),
                wechat_mini_program_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
