use std::fmt;

use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use hyacinth::schema::wechat_oauth2_users;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_value};

use super::super::super::{
    HttpError, Result,
    orm::postgresql::Connection,
    wechat::oauth2::responses::{AccessToken, RefreshToken, UserInfo},
};

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub union_id: String,
    pub app_id: String,
    pub open_id: String,
    pub nickname: String,
    pub head_img_url: Option<String>,
    pub privilege: Value,
    pub access_token: String,
    pub expires_in: NaiveDateTime,
    pub refresh_token: String,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Item {
    pub fn is_enable(&self) -> Result<()> {
        if self.locked_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::LOCKED,
                Some("User is locked".to_string()),
            )));
        }
        if self.deleted_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::GONE,
                Some("User is gone".to_string()),
            )));
        }
        Ok(())
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}<{}>", self.nickname, self.union_id)
    }
}

pub trait Dao {
    fn count(&mut self) -> Result<i64>;
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_app_and_open_id(&mut self, app_id: &str, open_id: &str) -> Result<Item>;
    fn by_union_id(&mut self, union_id: &str) -> Result<Vec<Item>>;
    fn create(
        &mut self,
        user: i64,
        app_id: &str,
        token: &AccessToken,
        info: &UserInfo,
    ) -> Result<()>;
    fn set_info(&mut self, id: i64, info: &UserInfo) -> Result<()>;
    fn set_access_token(&mut self, id: i64, token: &RefreshToken) -> Result<()>;
    fn lock(&mut self, id: i64) -> Result<()>;
    fn unlock(&mut self, id: i64) -> Result<()>;
    fn delete(&mut self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn count(&mut self) -> Result<i64> {
        let it: i64 = wechat_oauth2_users::dsl::wechat_oauth2_users
            .count()
            .get_result(self)?;
        Ok(it)
    }
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = wechat_oauth2_users::dsl::wechat_oauth2_users
            .order(wechat_oauth2_users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_app_and_open_id(&mut self, app_id: &str, open_id: &str) -> Result<Item> {
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::app_id.eq(app_id))
            .filter(wechat_oauth2_users::dsl::open_id.eq(open_id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_union_id(&mut self, union_id: &str) -> Result<Vec<Item>> {
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::union_id.eq(union_id))
            .load::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i64,
        app_id: &str,
        token: &AccessToken,
        info: &UserInfo,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let privilege = to_value(&info.privilege)?;
        insert_into(wechat_oauth2_users::dsl::wechat_oauth2_users)
            .values((
                wechat_oauth2_users::dsl::user_id.eq(user),
                wechat_oauth2_users::dsl::union_id.eq(&token.unionid),
                wechat_oauth2_users::dsl::app_id.eq(app_id),
                wechat_oauth2_users::dsl::open_id.eq(&token.openid),
                wechat_oauth2_users::dsl::access_token.eq(&token.access_token),
                wechat_oauth2_users::dsl::expires_in
                    .eq(now + Duration::seconds(token.expires_in as i64)),
                wechat_oauth2_users::dsl::refresh_token.eq(&token.refresh_token),
                wechat_oauth2_users::dsl::nickname.eq(&info.nickname),
                wechat_oauth2_users::dsl::head_img_url.eq(&info.headimgurl),
                wechat_oauth2_users::dsl::privilege.eq(&privilege),
                wechat_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_info(&mut self, id: i64, info: &UserInfo) -> Result<()> {
        let now = Utc::now().naive_utc();
        let privilege = to_value(&info.privilege)?;
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_oauth2_users::dsl::nickname.eq(&info.nickname),
                wechat_oauth2_users::dsl::head_img_url.eq(&info.headimgurl),
                wechat_oauth2_users::dsl::privilege.eq(&privilege),
                wechat_oauth2_users::dsl::version.eq(wechat_oauth2_users::dsl::version + 1),
                wechat_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_access_token(&mut self, id: i64, token: &RefreshToken) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_oauth2_users::dsl::access_token.eq(&token.access_token),
                wechat_oauth2_users::dsl::expires_in
                    .eq(now + Duration::seconds(token.expires_in as i64)),
                wechat_oauth2_users::dsl::refresh_token.eq(&token.refresh_token),
                wechat_oauth2_users::dsl::version.eq(wechat_oauth2_users::dsl::version + 1),
                wechat_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn lock(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_oauth2_users::dsl::locked_at.eq(&now),
                wechat_oauth2_users::dsl::version.eq(wechat_oauth2_users::dsl::version + 1),
                wechat_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_oauth2_users::dsl::locked_at.eq(None::<NaiveDateTime>),
                wechat_oauth2_users::dsl::version.eq(wechat_oauth2_users::dsl::version + 1),
                wechat_oauth2_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn delete(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::id.eq(id));
        update(it)
            .set(wechat_oauth2_users::dsl::deleted_at.eq(&now))
            .execute(self)?;
        Ok(())
    }
}
