use std::fmt;

use chrono::{NaiveDateTime, Utc};
use chrono_tz::Tz;
use diesel::{delete, insert_into, prelude::*, update};
use language_tags::LanguageTag;
use palm::{
    orchid::v1::{
        wechat_oauth2_login_response::Sex, wechat_oauth2_qr_connect_request::Language,
        WechatOauth2LoginResponse,
    },
    random::uuid,
    Result,
};
use serde::{Deserialize, Serialize};

use super::super::super::{orm::postgresql::Connection, schema::wechat_oauth2_users};
use super::super::user::{Dao as UserDao, Item as User};

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub union_id: String,
    pub app_id: String,
    pub open_id: String,
    pub nickname: String,
    pub sex: i32,
    pub city: String,
    pub province: String,
    pub country: String,
    pub head_img_url: Option<String>,
    pub privilege: Vec<u8>,
    pub lang: String,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn sex(&self) -> Result<String> {
        let it = Sex::try_from(self.sex)?;
        let it = it.as_str_name();
        Ok(it.to_string())
    }
    pub fn privilege(&self) -> Result<Vec<String>> {
        let it = flexbuffers::from_slice(&self.privilege)?;
        Ok(it)
    }
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
    fn set_profile(&mut self, id: i64, user_info: &WechatOauth2LoginResponse) -> Result<()>;
    fn sign_in(
        &mut self,
        user_id: Option<i64>,
        app_id: &str,
        user_info: &WechatOauth2LoginResponse,
        lang: Language,
        ip: &str,
        timezone: Tz,
    ) -> Result<User>;
    fn destroy(&mut self, id: i64) -> Result<()>;
    fn bind(&mut self, id: i64, user: i64) -> Result<()>;
    fn count_by_user(&mut self, user: i64) -> Result<i64>;
    fn count(&mut self) -> Result<i64>;
}

impl Dao for Connection {
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
    fn by_open_id(&mut self, app_id: &str, open_id: &str) -> Result<Item> {
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::app_id.eq(app_id))
            .filter(wechat_oauth2_users::dsl::open_id.eq(open_id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_union_id(&mut self, union_id: &str) -> Result<Item> {
        let it = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::union_id.eq(union_id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn set_profile(&mut self, id: i64, info: &WechatOauth2LoginResponse) -> Result<()> {
        let now = Utc::now().naive_utc();
        let user = Dao::by_id(self, id)?;
        if user.head_img_url != info.headimgurl {
            if let Some(ref avatar) = info.headimgurl {
                update(
                    wechat_oauth2_users::dsl::wechat_oauth2_users
                        .filter(wechat_oauth2_users::dsl::id.eq(id)),
                )
                .set((
                    wechat_oauth2_users::dsl::head_img_url.eq(avatar),
                    wechat_oauth2_users::dsl::updated_at.eq(&now),
                ))
                .execute(self)?;
            }
        }

        Ok(())
    }
    fn sign_in(
        &mut self,
        user_id: Option<i64>,
        app_id: &str,
        info: &WechatOauth2LoginResponse,
        lang: Language,
        ip: &str,
        timezone: Tz,
    ) -> Result<User> {
        let privilege = flexbuffers::to_vec(&info.privilege)?;
        let now = Utc::now().naive_utc();

        let user = match Dao::by_open_id(self, app_id, &info.openid) {
            Ok(ref it) => {
                update(
                    wechat_oauth2_users::dsl::wechat_oauth2_users
                        .filter(wechat_oauth2_users::dsl::id.eq(it.id)),
                )
                .set((
                    wechat_oauth2_users::dsl::nickname.eq(&info.nickname),
                    wechat_oauth2_users::dsl::sex.eq(info.sex),
                    wechat_oauth2_users::dsl::city.eq(&info.city),
                    wechat_oauth2_users::dsl::province.eq(&info.province),
                    wechat_oauth2_users::dsl::country.eq(&info.country),
                    wechat_oauth2_users::dsl::head_img_url.eq(&info.headimgurl),
                    wechat_oauth2_users::dsl::privilege.eq(&privilege),
                    wechat_oauth2_users::dsl::lang.eq(lang.as_str_name()),
                    wechat_oauth2_users::dsl::updated_at.eq(&now),
                ))
                .execute(self)?;
                Dao::set_profile(self, it.user_id, info)?;
                let user = UserDao::by_id(self, it.user_id)?;
                if !(user.name.is_some() && user.name.as_deref() == Some(&info.nickname)) {
                    UserDao::set_name(self, user.id, Some(&info.nickname))?;
                }
                if user.avatar != info.headimgurl {
                    UserDao::set_avatar(self, user.id, info.headimgurl.as_deref())?;
                }
                user
            }
            Err(_) => {
                let user = match user_id {
                    Some(id) => {
                        Dao::set_profile(self, id, info)?;
                        UserDao::by_id(self, id)?
                    }
                    None => {
                        let uid = uuid();
                        let lang = match lang {
                            Language::Tw => LanguageTag::parse("zh-Hant")?,
                            Language::Cn => LanguageTag::parse("zh-Hans")?,
                            Language::En => LanguageTag::parse("en-US")?,
                        };
                        UserDao::create(
                            self,
                            &uid,
                            Some(&info.nickname),
                            info.headimgurl.as_deref(),
                            &lang,
                            timezone,
                        )?;
                        UserDao::by_uid(self, &uid)?
                    }
                };
                insert_into(wechat_oauth2_users::dsl::wechat_oauth2_users)
                    .values((
                        wechat_oauth2_users::dsl::user_id.eq(user.id),
                        wechat_oauth2_users::dsl::app_id.eq(app_id),
                        wechat_oauth2_users::dsl::open_id.eq(&info.openid),
                        wechat_oauth2_users::dsl::union_id.eq(&info.unionid),
                        wechat_oauth2_users::dsl::nickname.eq(&info.nickname),
                        wechat_oauth2_users::dsl::sex.eq(info.sex),
                        wechat_oauth2_users::dsl::city.eq(&info.city),
                        wechat_oauth2_users::dsl::province.eq(&info.province),
                        wechat_oauth2_users::dsl::country.eq(&info.country),
                        wechat_oauth2_users::dsl::head_img_url.eq(&info.headimgurl),
                        wechat_oauth2_users::dsl::privilege.eq(&privilege),
                        wechat_oauth2_users::dsl::lang.eq(lang.as_str_name()),
                        wechat_oauth2_users::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
                user
            }
        };
        UserDao::sign_in(self, user.id, ip)?;
        Ok(user)
    }

    fn destroy(&mut self, id: i64) -> Result<()> {
        delete(
            wechat_oauth2_users::dsl::wechat_oauth2_users
                .filter(wechat_oauth2_users::dsl::id.eq(id)),
        )
        .execute(self)?;
        Ok(())
    }
    fn bind(&mut self, id: i64, user: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(
            wechat_oauth2_users::dsl::wechat_oauth2_users
                .filter(wechat_oauth2_users::dsl::id.eq(id)),
        )
        .set((
            wechat_oauth2_users::dsl::user_id.eq(user),
            wechat_oauth2_users::dsl::updated_at.eq(&now),
        ))
        .execute(self)?;
        Ok(())
    }
    fn count_by_user(&mut self, user: i64) -> Result<i64> {
        let cnt: i64 = wechat_oauth2_users::dsl::wechat_oauth2_users
            .filter(wechat_oauth2_users::dsl::user_id.eq(user))
            .count()
            .get_result(self)?;
        Ok(cnt)
    }
    fn count(&mut self) -> Result<i64> {
        let cnt: i64 = wechat_oauth2_users::dsl::wechat_oauth2_users
            .count()
            .get_result(self)?;
        Ok(cnt)
    }
}
