use std::fmt;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::super::schema::wechat_oauth2_users;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub union_id: String,
    pub app_id: String,
    pub open_id: String,
    pub nickname: String,
    pub sex: i16,
    pub city: String,
    pub province: String,
    pub country: String,
    pub head_img_url: Option<String>,
    pub privilege: Vec<u8>,
    pub lang: String,
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
        info: (&str, i16, &str, &str, &str, Option<&str>, &[u8], &str),
    ) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_open_id(&mut self, app_id: &str, open_id: &str) -> Result<Item>;
    fn by_union_id(&mut self, union_id: &str) -> Result<Item>;
    fn total(&mut self) -> Result<i64>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn update(
        &mut self,
        id: i32,
        info: (&str, i16, &str, &str, &str, Option<&str>, &[u8], &str),
    ) -> Result<()>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        user: i32,
        (union_id, app_id, open_id): (&str, &str, &str),
        (nickname, sex, city, province, country, head_img_url, privilege, lang): (
            &str,
            i16,
            &str,
            &str,
            &str,
            Option<&str>,
            &[u8],
            &str,
        ),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(wechat_oauth2_users::dsl::wechat_oauth2_users)
            .values((
                wechat_oauth2_users::dsl::user_id.eq(user),
                wechat_oauth2_users::dsl::union_id.eq(union_id),
                wechat_oauth2_users::dsl::app_id.eq(app_id),
                wechat_oauth2_users::dsl::open_id.eq(open_id),
                wechat_oauth2_users::dsl::nickname.eq(nickname),
                wechat_oauth2_users::dsl::sex.eq(sex),
                wechat_oauth2_users::dsl::city.eq(city),
                wechat_oauth2_users::dsl::province.eq(province),
                wechat_oauth2_users::dsl::country.eq(country),
                wechat_oauth2_users::dsl::head_img_url.eq(head_img_url),
                wechat_oauth2_users::dsl::privilege.eq(privilege),
                wechat_oauth2_users::dsl::lang.eq(lang),
                wechat_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }
    fn by_open_id(&mut self, app_id: &str, open_id: &str) -> Result<Item> {
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::app_id.eq(app_id))
            .filter(wechat_oauth2_users::dsl::open_id.eq(open_id))
            .first(self)?;
        Ok(it)
    }
    fn by_union_id(&mut self, union_id: &str) -> Result<Item> {
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::union_id.eq(union_id))
            .first(self)?;
        Ok(it)
    }
    fn total(&mut self) -> Result<i64> {
        let it: i64 = wechat_oauth2_users::dsl::wechat_oauth2_users
            .count()
            .get_result(self)?;
        Ok(it)
    }
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = wechat_oauth2_users::dsl::wechat_oauth2_users
            .order(wechat_oauth2_users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_oauth2_users::dsl::deleted_at.eq(&Some(now)),
                wechat_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_oauth2_users::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                wechat_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(
        &mut self,
        id: i32,
        (nickname, sex, city, province, country, head_img_url, privilege, lang): (
            &str,
            i16,
            &str,
            &str,
            &str,
            Option<&str>,
            &[u8],
            &str,
        ),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_oauth2_users::dsl::nickname.eq(nickname),
                wechat_oauth2_users::dsl::sex.eq(sex),
                wechat_oauth2_users::dsl::city.eq(city),
                wechat_oauth2_users::dsl::province.eq(province),
                wechat_oauth2_users::dsl::country.eq(country),
                wechat_oauth2_users::dsl::head_img_url.eq(head_img_url),
                wechat_oauth2_users::dsl::privilege.eq(privilege),
                wechat_oauth2_users::dsl::lang.eq(lang),
                wechat_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
