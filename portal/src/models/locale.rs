use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use hyacinth::schema::locales;

use super::super::{Result, orm::postgresql::Connection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = locales)]
pub struct New<'a> {
    pub lang: &'a str,
    pub code: &'a str,
    pub message: &'a str,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn languages(&mut self) -> Result<Vec<String>>;
    fn count_by_lang(&mut self, lang: &str) -> Result<i64>;
    fn count(&mut self) -> Result<i64>;
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_lang(&mut self, lang: &str) -> Result<Vec<Item>>;
    fn by_code(&mut self, code: &str) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_lang_and_code(&mut self, lang: &str, code: &str) -> Result<Item>;
    fn delete(&mut self, id: i64) -> Result<()>;
    fn create(&mut self, lang: &str, code: &str, message: &str) -> Result<()>;
    fn update(&mut self, id: i64, message: &str) -> Result<()>;
}

impl Dao for Connection {
    fn languages(&mut self) -> Result<Vec<String>> {
        Ok(locales::dsl::locales
            .select(locales::dsl::lang)
            .distinct()
            .order(locales::dsl::lang.asc())
            .load::<String>(self)?)
    }

    fn count(&mut self) -> Result<i64> {
        let it: i64 = locales::dsl::locales.count().get_result(self)?;
        Ok(it)
    }
    fn count_by_lang(&mut self, lang: &str) -> Result<i64> {
        let cnt: i64 = locales::dsl::locales
            .count()
            .filter(locales::dsl::lang.eq(lang))
            .get_result(self)?;
        Ok(cnt)
    }
    fn by_lang(&mut self, lang: &str) -> Result<Vec<Item>> {
        let items = locales::dsl::locales
            .filter(locales::dsl::lang.eq(lang))
            .order(locales::dsl::code.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_code(&mut self, code: &str) -> Result<Vec<Item>> {
        let items = locales::dsl::locales
            .filter(locales::dsl::code.eq(code))
            .order(locales::dsl::lang.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = locales::dsl::locales
            .order(locales::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = locales::dsl::locales
            .filter(locales::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_lang_and_code(&mut self, lang: &str, code: &str) -> Result<Item> {
        let it = locales::dsl::locales
            .filter(locales::dsl::lang.eq(lang))
            .filter(locales::dsl::code.eq(code))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn update(&mut self, id: i64, message: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = locales::dsl::locales.filter(locales::dsl::id.eq(id));
        update(it)
            .set((
                locales::dsl::message.eq(message),
                locales::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn create(&mut self, lang: &str, code: &str, message: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(locales::dsl::locales)
            .values(&New {
                lang,
                code,
                message,
                updated_at: &now,
            })
            .execute(self)?;
        Ok(())
    }
    fn delete(&mut self, id: i64) -> Result<()> {
        delete(locales::dsl::locales.filter(locales::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
