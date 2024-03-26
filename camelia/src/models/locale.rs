use std::fs::read_dir;
use std::path::Path;

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use ini::Ini;
use log::{debug, info, warn};
use palm::Result;
use serde::{Deserialize, Serialize};

use super::super::{orm::postgresql::Connection, schema::locales};

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    fn sync<P: AsRef<Path>>(&mut self, root: P) -> Result<(usize, usize)>;
    fn languages(&mut self) -> Result<Vec<String>>;
    fn count_by_lang(&mut self, lang: &str) -> Result<i64>;
    fn count(&mut self) -> Result<i64>;
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_lang(&mut self, lang: &str) -> Result<Vec<Item>>;
    fn by_code(&mut self, code: &str) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_lang_and_code(&mut self, lang: &str, code: &str) -> Result<Item>;
    fn delete(&mut self, id: i32) -> Result<()>;
    fn create(&mut self, lang: &str, code: &str, message: &str) -> Result<()>;
    fn update(&mut self, id: i32, message: &str) -> Result<()>;
}

fn load_from_ini<P: AsRef<Path>>(
    db: &mut Connection,
    lang: &str,
    file: P,
) -> Result<(usize, usize)> {
    let mut found = 0;
    let mut inserted = 0;
    let file = file.as_ref();
    info!("find file {}", file.display());

    for (sec, props) in Ini::load_from_file(file)?.iter() {
        if let Some(sec) = sec {
            debug!("find section {}", sec);
            for (key, val) in props.iter() {
                let code = format!("{}.{}", sec, key);
                debug!("find {lang}.{code} = {val}");
                found += 1;
                let cnt: i64 = locales::dsl::locales
                    .count()
                    .filter(locales::dsl::lang.eq(lang))
                    .filter(locales::dsl::code.eq(&code))
                    .get_result(db)?;
                if cnt == 0 {
                    Dao::create(db, lang, &code, val)?;
                    inserted += 1;
                } else {
                    warn!("{lang}.{code} already exists!");
                }
            }
        }
    }

    Ok((found, inserted))
}

impl Dao for Connection {
    fn sync<P: AsRef<Path>>(&mut self, root: P) -> Result<(usize, usize)> {
        let root = root.as_ref();
        info!("load items from {}", root.display());

        let mut found = 0;
        let mut inserted = 0;

        for it in read_dir(root)? {
            let it = it?;
            let it = it.path();

            if it.is_dir() {
                if let Some(lang) = it.file_name() {
                    if let Some(lang) = lang.to_str() {
                        info!("find language {}", lang);
                        for it in read_dir(&it)? {
                            let it = it?;
                            let it = it.path();
                            if it.is_file() {
                                let (f, i) = load_from_ini(self, lang, it)?;
                                found += f;
                                inserted += i;
                            }
                        }
                    }
                }
            }
        }
        for lang in self.languages()?.iter() {
            for it in read_dir(root)? {
                let it = it?;
                let it = it.path();

                if it.is_file() {
                    let (f, i) = load_from_ini(self, lang, it)?;
                    found += f;
                    inserted += i;
                }
            }
        }

        info!("sync {}/{} items", inserted, found);
        Ok((inserted, found))
    }

    fn languages(&mut self) -> Result<Vec<String>> {
        Ok(locales::dsl::locales
            .select(locales::dsl::lang)
            .distinct()
            .order(locales::dsl::lang.asc())
            .load::<String>(self)?)
    }

    fn count(&mut self) -> Result<i64> {
        let cnt: i64 = locales::dsl::locales.count().get_result(self)?;
        Ok(cnt)
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
            .order((locales::dsl::code.asc(), locales::dsl::lang.asc()))
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
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
    fn update(&mut self, id: i32, message: &str) -> Result<()> {
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
    fn delete(&mut self, id: i32) -> Result<()> {
        delete(locales::dsl::locales.filter(locales::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
