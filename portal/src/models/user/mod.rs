pub mod email;
pub mod google_oauth2;
pub mod wechat_mini_program;
pub mod wechat_oauth2;

use chrono::{NaiveDateTime, Utc};
use chrono_tz::Tz;
use diesel::{insert_into, prelude::*, update};
use hyacinth::schema::users;
use icu::locale::Locale;

use super::super::{Result, orm::postgresql::Connection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub uid: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
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

pub trait Dao {
    fn count(&mut self) -> Result<i64>;
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_uid(&mut self, uid: &str) -> Result<Item>;
    fn create(&mut self, uid: &str, lang: &Locale, timezone: Tz) -> Result<()>;
    fn set_lang(&mut self, id: i64, lang: &Locale) -> Result<()>;
    fn set_timezone(&mut self, id: i64, timezone: Tz) -> Result<()>;
    fn set_name(&mut self, id: i64, name: Option<&str>) -> Result<()>;
    fn set_avatar(&mut self, id: i64, avatar: Option<&str>) -> Result<()>;
    fn sign_in(&mut self, id: i64, ip: &str) -> Result<()>;
    fn lock(&mut self, id: i64) -> Result<()>;
    fn unlock(&mut self, id: i64) -> Result<()>;
    fn delete(&mut self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn count(&mut self) -> Result<i64> {
        let it: i64 = users::dsl::users.count().get_result(self)?;
        Ok(it)
    }
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = users::dsl::users
            .order(users::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = users::dsl::users
            .filter(users::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_uid(&mut self, uid: &str) -> Result<Item> {
        let it = users::dsl::users
            .filter(users::dsl::uid.eq(uid))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(&mut self, uid: &str, lang: &Locale, timezone: Tz) -> Result<()> {
        let timezone = timezone.to_string();
        let lang = lang.to_string();
        let now = Utc::now().naive_utc();
        insert_into(users::dsl::users)
            .values((
                users::dsl::uid.eq(uid),
                users::dsl::lang.eq(&lang),
                users::dsl::timezone.eq(&timezone),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_lang(&mut self, id: i64, lang: &Locale) -> Result<()> {
        let lang = lang.to_string();
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::lang.eq(&lang),
                users::dsl::version.eq(users::dsl::version + 1),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_timezone(&mut self, id: i64, timezone: Tz) -> Result<()> {
        let timezone = timezone.to_string();
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::timezone.eq(&timezone),
                users::dsl::version.eq(users::dsl::version + 1),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_name(&mut self, id: i64, name: Option<&str>) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::name.eq(name),
                users::dsl::version.eq(users::dsl::version + 1),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_avatar(&mut self, id: i64, avatar: Option<&str>) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::avatar.eq(avatar),
                users::dsl::version.eq(users::dsl::version + 1),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn sign_in(&mut self, id: i64, ip: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::current_sign_in_at.eq(&now),
                users::dsl::current_sign_in_ip.eq(ip),
                users::dsl::last_sign_in_at.eq(users::dsl::current_sign_in_at),
                users::dsl::last_sign_in_ip.eq(users::dsl::current_sign_in_ip),
                users::dsl::version.eq(users::dsl::version + 1),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn lock(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::locked_at.eq(&now),
                users::dsl::version.eq(users::dsl::version + 1),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unlock(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::locked_at.eq(None::<NaiveDateTime>),
                users::dsl::version.eq(users::dsl::version + 1),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn delete(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set(users::dsl::deleted_at.eq(&now))
            .execute(self)?;
        Ok(())
    }
}
