use std::mem;

use chrono::{NaiveDateTime, Utc};
use data_encoding::BASE64URL_NOPAD;
use diesel::{delete, insert_into, prelude::*, update};
use hyper::StatusCode;
use palm::{HttpError, Result};
use serde::Serialize;

use super::super::{orm::postgresql::Connection, schema::shorter_links};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub url: String,
    pub summary: String,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Item {
    pub fn uid(&self) -> String {
        let buf = self.id.to_ne_bytes();
        BASE64URL_NOPAD.encode(&buf)
    }
    pub fn parse_uid(s: &str) -> Result<i64> {
        let mut buf: [u8; mem::size_of::<i64>()] = Default::default();
        {
            let l = mem::size_of::<i64>();
            let it = BASE64URL_NOPAD.decode(s.as_bytes())?;
            if it.len() < l {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some(format!("bad id {}", s)),
                )));
            }
            buf.copy_from_slice(&it[0..l]);
        };
        Ok(i64::from_ne_bytes(buf))
    }
}

pub trait Dao {
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn create(&mut self, url: &str, summary: &str) -> Result<()>;
    fn update(&mut self, id: i64, url: &str, summary: &str) -> Result<()>;
    fn all(&mut self) -> Result<Vec<Item>>;
    fn destroy(&mut self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i64) -> Result<Item> {
        Ok(shorter_links::dsl::shorter_links
            .filter(shorter_links::dsl::id.eq(id))
            .first::<Item>(self)?)
    }

    fn create(&mut self, url: &str, summary: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(shorter_links::dsl::shorter_links)
            .values((
                shorter_links::dsl::url.eq(url),
                shorter_links::dsl::summary.eq(summary),
                shorter_links::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&mut self, id: i64, url: &str, summary: &str) -> Result<()> {
        let it = shorter_links::dsl::shorter_links.filter(shorter_links::dsl::id.eq(&id));
        let now = Utc::now().naive_utc();
        update(it)
            .set((
                shorter_links::dsl::url.eq(url),
                shorter_links::dsl::summary.eq(summary),
                shorter_links::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn all(&mut self) -> Result<Vec<Item>> {
        Ok(shorter_links::dsl::shorter_links
            .order(shorter_links::dsl::updated_at.desc())
            .load::<Item>(self)?)
    }

    fn destroy(&mut self, id: i64) -> Result<()> {
        delete(shorter_links::dsl::shorter_links.filter(shorter_links::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
}
