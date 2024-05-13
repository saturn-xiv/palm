use std::fmt;

use chrono::{NaiveDateTime, Utc};
use chrono_tz::Tz;
use diesel::{delete, insert_into, prelude::*, update};
use language_tags::LanguageTag;
use palm::{random::uuid, Result};
use serde::{Deserialize, Serialize};

use super::super::super::{orm::postgresql::Connection, schema::wechat_mini_program_users};
use super::super::user::Dao as UserDao;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub union_id: String,
    pub app_id: String,
    pub open_id: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
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
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_open_id(&mut self, app_id: &str, open_id: &str) -> Result<Item>;
    fn by_union_id(&mut self, union_id: &str) -> Result<Item>;
    fn by_user(&mut self, user: i64) -> Result<Vec<Item>>;
    fn bind(&mut self, id: i64, user: i64) -> Result<()>;
    fn set_profile(&mut self, id: i64, nickname: &str, avatar_url: &str) -> Result<()>;
    fn sign_in(
        &mut self,
        app_id: &str,
        open_id: &str,
        union_id: &str,
        ip: &str,
        lang: &LanguageTag,
        timezone: Tz,
    ) -> Result<Item>;
    fn count_by_user(&mut self, user: i64) -> Result<i64>;
    fn count(&mut self) -> Result<i64>;
    fn destroy(&mut self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = wechat_mini_program_users::dsl::wechat_mini_program_users
            .order(wechat_mini_program_users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_open_id(&mut self, app_id: &str, open_id: &str) -> Result<Item> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::app_id.eq(app_id))
            .filter(wechat_mini_program_users::dsl::open_id.eq(open_id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_user(&mut self, user: i64) -> Result<Vec<Item>> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::user_id.eq(user))
            .load::<Item>(self)?;
        Ok(it)
    }
    fn by_union_id(&mut self, union_id: &str) -> Result<Item> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::union_id.eq(union_id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn bind(&mut self, id: i64, user: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(
            wechat_mini_program_users::dsl::wechat_mini_program_users
                .filter(wechat_mini_program_users::dsl::id.eq(id)),
        )
        .set((
            wechat_mini_program_users::dsl::user_id.eq(user),
            wechat_mini_program_users::dsl::updated_at.eq(&now),
        ))
        .execute(self)?;
        Ok(())
    }
    fn set_profile(&mut self, id: i64, nickname: &str, avatar_url: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(
            wechat_mini_program_users::dsl::wechat_mini_program_users
                .filter(wechat_mini_program_users::dsl::id.eq(id)),
        )
        .set((
            wechat_mini_program_users::dsl::nickname.eq(nickname),
            wechat_mini_program_users::dsl::avatar_url.eq(avatar_url),
            wechat_mini_program_users::dsl::updated_at.eq(&now),
        ))
        .execute(self)?;
        Ok(())
    }
    fn sign_in(
        &mut self,
        app_id: &str,
        open_id: &str,
        union_id: &str,
        ip: &str,
        lang: &LanguageTag,
        timezone: Tz,
    ) -> Result<Item> {
        let now = Utc::now().naive_utc();
        let user = match wechat_mini_program_users::dsl::wechat_mini_program_users
            .select((
                wechat_mini_program_users::dsl::id,
                wechat_mini_program_users::dsl::user_id,
            ))
            .filter(wechat_mini_program_users::dsl::app_id.eq(app_id))
            .filter(wechat_mini_program_users::dsl::open_id.eq(open_id))
            .first::<(i64, i64)>(self)
        {
            Ok((_, user)) => user,
            Err(_) => {
                let uid = uuid();
                UserDao::create(self, &uid, lang, timezone)?;
                let user = UserDao::by_uid(self, &uid)?;
                insert_into(wechat_mini_program_users::dsl::wechat_mini_program_users)
                    .values((
                        wechat_mini_program_users::dsl::app_id.eq(app_id),
                        wechat_mini_program_users::dsl::open_id.eq(open_id),
                        wechat_mini_program_users::dsl::union_id.eq(union_id),
                        wechat_mini_program_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
                user.id
            }
        };
        UserDao::sign_in(self, user, ip)?;

        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::app_id.eq(app_id))
            .filter(wechat_mini_program_users::dsl::open_id.eq(open_id))
            .first::<Item>(self)?;
        Ok(it)
    }

    fn count_by_user(&mut self, user: i64) -> Result<i64> {
        let cnt: i64 = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::user_id.eq(user))
            .count()
            .get_result(self)?;
        Ok(cnt)
    }
    fn count(&mut self) -> Result<i64> {
        let cnt: i64 = wechat_mini_program_users::dsl::wechat_mini_program_users
            .count()
            .get_result(self)?;
        Ok(cnt)
    }
    fn destroy(&mut self, id: i64) -> Result<()> {
        delete(
            wechat_mini_program_users::dsl::wechat_mini_program_users
                .filter(wechat_mini_program_users::dsl::id.eq(id)),
        )
        .execute(self)?;
        Ok(())
    }
}
