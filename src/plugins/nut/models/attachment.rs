use std::path::Path;

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use mime::Mime;
use serde::Serialize;

use super::super::super::super::{orm::postgresql::Connection, Result};
use super::schema::attachments;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub size: i64,
    pub content_type: String,
    pub url: String,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn detect(title: &str) -> (String, String, mime::Mime) {
        let now = Utc::now();
        let bucket = now.format("%F").to_string();

        let mut name = Path::new(&now.format("%H-%M-%S-%f").to_string()).to_path_buf();
        if let Some(ext) = Path::new(title).extension() {
            name = name.with_extension(ext);
        }

        (
            bucket,
            name.display().to_string(),
            mime::APPLICATION_OCTET_STREAM,
        )
    }
}

pub trait Dao {
    fn by_id(&self, id: i64) -> Result<Item>;
    fn create(
        &self,
        user: i64,
        title: &str,
        content_type: &Mime,
        url: &str,
        size: usize,
    ) -> Result<()>;
    fn update(
        &self,
        id: i64,
        title: &str,
        content_type: &Mime,
        url: &str,
        size: usize,
    ) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn by_user(&self, user: i64) -> Result<Vec<Item>>;
    fn delete(&self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: i64) -> Result<Item> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &self,
        user: i64,
        title: &str,
        content_type: &Mime,
        url: &str,
        size: usize,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let content_type = content_type.to_string();
        insert_into(attachments::dsl::attachments)
            .values((
                attachments::dsl::user_id.eq(user),
                attachments::dsl::title.eq(title),
                attachments::dsl::content_type.eq(content_type),
                attachments::dsl::url.eq(url),
                attachments::dsl::size.eq(size as i64),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(
        &self,
        id: i64,
        title: &str,
        content_type: &Mime,
        url: &str,
        size: usize,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let content_type = content_type.to_string();
        update(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id)))
            .set((
                attachments::dsl::title.eq(title),
                attachments::dsl::content_type.eq(content_type),
                attachments::dsl::url.eq(url),
                attachments::dsl::size.eq(size as i64),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .order(attachments::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn by_user(&self, user: i64) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq(user))
            .order(attachments::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: i64) -> Result<()> {
        delete(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
