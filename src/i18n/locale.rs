use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use serde::Serialize;
use yaml_rust::{Yaml, YamlLoader};

use super::super::{
    orm::{migration::Migration, postgresql::Connection},
    Result,
};
use super::{schema::locales, DOWN, UP};

lazy_static! {
    pub static ref MIGRATION: Migration<'static> = Migration {
        name: "create-locales",
        version: "20210516151734",
        up: UP,
        down: DOWN
    };
}

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "locales"]
pub struct New<'a> {
    pub lang: &'a str,
    pub code: &'a str,
    pub message: &'a str,
    pub updated_at: &'a NaiveDateTime,
}

pub struct File<'a> {
    pub lang: &'a str,
    pub body: &'a str,
}

pub trait Dao {
    fn sync(&self, files: &[File]) -> Result<(usize, usize)>;
    fn languages(&self) -> Result<Vec<String>>;
    fn count(&self, lang: &str) -> Result<i64>;
    fn all(&self) -> Result<Vec<Item>>;
    fn by_lang(&self, lang: &str) -> Result<Vec<Item>>;
    fn by_id(&self, id: i64) -> Result<Item>;
    fn by_lang_and_code(&self, lang: &str, code: &str) -> Result<Item>;
    fn delete(&self, id: i64) -> Result<()>;
    fn create(&self, lang: &str, code: &str, message: &str) -> Result<()>;
    fn update(&self, id: i64, code: &str, message: &str) -> Result<()>;
}

fn loop_yaml(
    db: &Connection,
    lang: &str,
    prefix: Option<String>,
    node: Yaml,
) -> Result<(usize, usize)> {
    let mut find = 0;
    let mut inserted = 0;
    let sep = ".";
    match node {
        Yaml::String(v) => {
            let k = match prefix {
                Some(p) => p,
                None => "".to_string(),
            };
            // debug!("find {} {} => {}", lang, k, v);
            find += 1;

            let cnt: i64 = locales::dsl::locales
                .count()
                .filter(locales::dsl::lang.eq(lang))
                .filter(locales::dsl::code.eq(&k))
                .get_result(db)?;
            if cnt == 0 {
                inserted += 1;
                insert_into(locales::dsl::locales)
                    .values(&New {
                        lang,
                        code: &k,
                        message: &v,
                        updated_at: &Utc::now().naive_utc(),
                    })
                    .execute(db)?;
            }
        }
        Yaml::Hash(h) => {
            for (k, v) in h {
                match k {
                    Yaml::String(k) => {
                        let (i, f) = loop_yaml(
                            db,
                            lang,
                            Some(match prefix {
                                Some(ref p) => {
                                    let mut v = String::new();
                                    v.push_str(p);
                                    v.push_str(sep);
                                    v.push_str(&k);
                                    v
                                }
                                None => k,
                            }),
                            v,
                        )?;
                        inserted += i;
                        find += f;
                    }
                    k => {
                        error!("bad key {:?}", k);
                    }
                }
            }
        }
        k => {
            error!("bad key {:?}", k);
        }
    };
    Ok((inserted, find))
}

impl Dao for Connection {
    fn sync(&self, items: &[File]) -> Result<(usize, usize)> {
        let mut find = 0;
        let mut inserted = 0;

        for it in items {
            info!("find locale {}", it.lang);
            for node in YamlLoader::load_from_str(it.body)? {
                let (i, f) = loop_yaml(self, it.lang, None, node)?;
                inserted += i;
                find += f;
            }
        }

        Ok((inserted, find))
    }

    fn languages(&self) -> Result<Vec<String>> {
        Ok(locales::dsl::locales
            .select(locales::dsl::lang)
            .distinct()
            .load::<String>(self)?)
    }

    fn count(&self, lang: &str) -> Result<i64> {
        let cnt: i64 = locales::dsl::locales
            .count()
            .filter(locales::dsl::lang.eq(lang))
            .get_result(self)?;
        Ok(cnt)
    }
    fn by_lang(&self, lang: &str) -> Result<Vec<Item>> {
        let items = locales::dsl::locales
            .filter(locales::dsl::lang.eq(lang))
            .order(locales::dsl::code.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn all(&self) -> Result<Vec<Item>> {
        let items = locales::dsl::locales
            .order(locales::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&self, id: i64) -> Result<Item> {
        let it = locales::dsl::locales
            .filter(locales::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_lang_and_code(&self, lang: &str, code: &str) -> Result<Item> {
        let it = locales::dsl::locales
            .filter(locales::dsl::lang.eq(lang))
            .filter(locales::dsl::code.eq(code))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn update(&self, id: i64, code: &str, message: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = locales::dsl::locales.filter(locales::dsl::id.eq(id));
        update(it)
            .set((
                locales::dsl::code.eq(code),
                locales::dsl::message.eq(message),
                locales::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn create(&self, lang: &str, code: &str, message: &str) -> Result<()> {
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
    fn delete(&self, id: i64) -> Result<()> {
        delete(locales::dsl::locales.filter(locales::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
