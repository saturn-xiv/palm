use std::fmt;
use std::result::Result as StdResult;

use chrono::{NaiveDateTime, Utc};
use chrono_tz::Tz;
use diesel::{insert_into, prelude::*, result::Error as DieselError, update};
use hyacinth::schema::wechat_mini_program_users;
use hyper::StatusCode;
use icu::locale::Locale;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::super::{
    HttpError, Result, orm::postgresql::Connection,
    wechat::mini_program::code2session::Response as Code2SessionResponse,
};
use super::Dao as UserDao;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub uid: String,
    pub union_id: String,
    pub app_id: String,
    pub open_id: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub session_key: String,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Item {
    pub fn subject(&self) -> String {
        format!("{}.{}", self.app_id, self.open_id)
    }
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
        write!(
            f,
            "{}<{}>",
            self.nickname.as_deref().unwrap_or_default(),
            self.union_id
        )
    }
}

pub trait Dao {
    fn count(&mut self) -> Result<i64>;
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_user(&mut self, id: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn by_app_and_open_id(&mut self, app_id: &str, open_id: &str) -> StdResult<Item, DieselError>;
    fn by_union_id(&mut self, union_id: &str) -> Result<Vec<Item>>;
    fn create(
        &mut self,
        user: i64,
        app_id: &str,
        open_id: &str,
        union_id: &str,
        session_key: &str,
    ) -> StdResult<(), DieselError>;
    fn set_info(&mut self, id: i64, nickname: &str, avatar_url: &str) -> Result<()>;
    fn set_session_key(&mut self, id: i64, session_key: &str) -> StdResult<(), DieselError>;
    fn lock(&mut self, id: i64) -> Result<()>;
    fn unlock(&mut self, id: i64) -> Result<()>;
    fn delete(&mut self, id: i64) -> Result<()>;
    fn sign_in_or_up(
        &mut self,
        location: (&Locale, Tz),
        info: (&str, &Code2SessionResponse),
    ) -> Result<Item>;
}

impl Dao for Connection {
    fn count(&mut self) -> Result<i64> {
        let it: i64 = wechat_mini_program_users::dsl::wechat_mini_program_users
            .count()
            .get_result(self)?;
        Ok(it)
    }
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = wechat_mini_program_users::dsl::wechat_mini_program_users
            .order(wechat_mini_program_users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_user(&mut self, id: i64) -> Result<Vec<Item>> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .order(wechat_mini_program_users::dsl::updated_at.desc())
            .filter(wechat_mini_program_users::dsl::user_id.eq(id))
            .load::<Item>(self)?;
        Ok(it)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::uid.eq(uid))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_app_and_open_id(&mut self, app_id: &str, open_id: &str) -> StdResult<Item, DieselError> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::app_id.eq(app_id))
            .filter(wechat_mini_program_users::dsl::open_id.eq(open_id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_union_id(&mut self, union_id: &str) -> Result<Vec<Item>> {
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::union_id.eq(union_id))
            .load::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i64,
        app_id: &str,
        open_id: &str,
        union_id: &str,
        session_key: &str,
    ) -> StdResult<(), DieselError> {
        let now = Utc::now().naive_utc();
        let uid = Uuid::new_v4().to_string();

        insert_into(wechat_mini_program_users::dsl::wechat_mini_program_users)
            .values((
                wechat_mini_program_users::dsl::uid.eq(&uid),
                wechat_mini_program_users::dsl::user_id.eq(user),
                wechat_mini_program_users::dsl::union_id.eq(union_id),
                wechat_mini_program_users::dsl::app_id.eq(app_id),
                wechat_mini_program_users::dsl::open_id.eq(open_id),
                wechat_mini_program_users::dsl::session_key.eq(session_key),
                wechat_mini_program_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_info(&mut self, id: i64, nickname: &str, avatar_url: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_mini_program_users::dsl::nickname.eq(nickname),
                wechat_mini_program_users::dsl::avatar_url.eq(avatar_url),
                wechat_mini_program_users::dsl::version
                    .eq(wechat_mini_program_users::dsl::version + 1),
                wechat_mini_program_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_session_key(&mut self, id: i64, session_key: &str) -> StdResult<(), DieselError> {
        let now = Utc::now().naive_utc();
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_mini_program_users::dsl::session_key.eq(session_key),
                wechat_mini_program_users::dsl::version
                    .eq(wechat_mini_program_users::dsl::version + 1),
                wechat_mini_program_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn lock(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_mini_program_users::dsl::locked_at.eq(&now),
                wechat_mini_program_users::dsl::version
                    .eq(wechat_mini_program_users::dsl::version + 1),
                wechat_mini_program_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id));
        update(it)
            .set((
                wechat_mini_program_users::dsl::locked_at.eq(None::<NaiveDateTime>),
                wechat_mini_program_users::dsl::version
                    .eq(wechat_mini_program_users::dsl::version + 1),
                wechat_mini_program_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn delete(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = wechat_mini_program_users::dsl::wechat_mini_program_users
            .filter(wechat_mini_program_users::dsl::id.eq(id));
        update(it)
            .set(wechat_mini_program_users::dsl::deleted_at.eq(&now))
            .execute(self)?;
        Ok(())
    }

    fn sign_in_or_up(
        &mut self,
        (lang, timezone): (&Locale, Tz),
        (app_id, user_info): (&str, &Code2SessionResponse),
    ) -> Result<Item> {
        match self.by_app_and_open_id(app_id, &user_info.openid) {
            Ok(it) => {
                it.is_enable()?;
                if it.union_id != user_info.unionid {
                    return Err(Box::new(HttpError(
                        StatusCode::BAD_REQUEST,
                        Some("invalid union_id".to_string()),
                    )));
                }
                {
                    let user = UserDao::by_id(self, it.user_id)?;
                    user.is_enable()?;
                }
                self.set_session_key(it.id, &user_info.session_key)?;
                Ok(())
            }
            Err(DieselError::NotFound) => {
                let uid = Uuid::new_v4().to_string();
                UserDao::create(self, &uid, "Wechat MiniProgram User", lang, timezone)?;
                let user = UserDao::by_uid(self, &uid)?;
                Dao::create(
                    self,
                    user.id,
                    app_id,
                    &user_info.openid,
                    &user_info.unionid,
                    &user_info.session_key,
                )?;
                Ok(())
            }
            Err(e) => Err(e),
        }?;
        Ok(self.by_app_and_open_id(app_id, &user_info.openid)?)
    }
}
