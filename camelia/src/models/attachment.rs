use std::any::type_name;
use std::fmt;
use std::path::Path;
use std::string::ToString;

use actix_files::file_extension_to_mime;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use hibiscus::{jasmine::S3, HttpError, Result};
use hyper::StatusCode;
use log::warn;
use mime::{Mime, IMAGE};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};
use uuid::Uuid;

use super::super::{
    orm::postgresql::Connection,
    schema::{attachment_resources, attachments},
};

pub fn cover<T>(db: &mut Connection, id: i64) -> Result<Item> {
    let it = Dao::by_resource::<T>(db, id)?
        .into_iter()
        .find(|x| x.is_picture())
        .ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("empty cover".to_string()),
        )))?;
    Ok(it)
}

#[derive(Queryable, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub bucket: String,
    pub name: String,
    pub title: String,
    pub size: i64,
    pub content_type: String,
    pub status: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl fmt::Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}<{},{}>", self.title, self.bucket, self.name)
    }
}
impl Item {
    pub fn url<S: S3>(&self, s3: &S, ttl: Option<i64>) -> Result<String> {
        let it = match self.status.parse::<Status>()? {
            Status::Public => s3.get_permanent_url(&self.bucket, &self.name)?,
            Status::Private => s3.get_presigned_url(
                &self.bucket,
                &self.name,
                &self.title,
                Duration::try_seconds(ttl.unwrap_or(60 * 60 * 24)).ok_or(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("bad ttl".to_string()),
                )))?,
            )?,
        };
        Ok(it)
    }
    pub fn size(body: &[u8]) -> usize {
        body.len() / (1 << 10)
    }

    pub fn name(title: &str) -> String {
        let id = Uuid::new_v4();
        if let Some(it) = Path::new(&title).extension() {
            if let Some(it) = it.to_str() {
                return format!("{}.{}", id, it);
            }
        }
        id.to_string()
    }
    pub fn content_type(title: &str) -> Mime {
        if let Some(it) = Path::new(&title).extension() {
            if let Some(it) = it.to_str() {
                return file_extension_to_mime(it);
            }
        }
        warn!("empty extension {}", title);
        mime::APPLICATION_OCTET_STREAM
    }
    pub fn is_picture(&self) -> bool {
        if let Ok(ref it) = self.content_type.parse::<Mime>() {
            return it.type_() == IMAGE;
        }
        false
    }
}

#[derive(EnumString, EnumDisplay, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Public,
    Private,
}

pub trait Dao {
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_bucket_and_name(&mut self, bucket: &str, name: &str) -> Result<Item>;
    fn create(
        &mut self,
        user: i64,
        bucket: &str,
        name: &str,
        title: &str,
        content_type: &Mime,
        size: u64,
    ) -> Result<()>;
    fn update(&mut self, id: i64, title: &str) -> Result<()>;
    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count(&mut self) -> Result<i64>;
    fn by_user(&mut self, user: i64, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count_by_user(&mut self, user: i64) -> Result<i64>;
    fn delete(&mut self, id: i64) -> Result<()>;
    fn associate_(&mut self, id: i64, resource_type: &str, resource_id: i64) -> Result<()>;
    fn dissociate_(&mut self, id: i64, resource_type: &str, resource_id: i64) -> Result<()>;
    fn by_resource_(&mut self, resource_type: &str, resource_id: i64) -> Result<Vec<Item>>;
    fn clear_(&mut self, resource_type: &str, resource_id: i64) -> Result<()>;
    fn associate<T>(&mut self, id: i64, resource_id: i64) -> Result<()>;
    fn dissociate<T>(&mut self, id: i64, resource_id: i64) -> Result<()>;
    fn clear<T>(&mut self, resource_id: i64) -> Result<()>;
    fn by_resource<T>(&mut self, resource_id: i64) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_bucket_and_name(&mut self, bucket: &str, name: &str) -> Result<Item> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::bucket.eq(bucket))
            .filter(attachments::dsl::name.eq(name))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i64,
        bucket: &str,
        name: &str,
        title: &str,
        content_type: &Mime,
        size: u64,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();

        let content_type = content_type.to_string();
        insert_into(attachments::dsl::attachments)
            .values((
                attachments::dsl::user_id.eq(user),
                attachments::dsl::bucket.eq(bucket),
                attachments::dsl::name.eq(name),
                attachments::dsl::title.eq(title),
                attachments::dsl::content_type.eq(content_type),
                attachments::dsl::size.eq(size as i64),
                attachments::dsl::status.eq(Status::Private.to_string()),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(&mut self, id: i64, title: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id)))
            .set((
                attachments::dsl::title.eq(title),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .order(attachments::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn count(&mut self) -> Result<i64> {
        let it = attachments::dsl::attachments.count().first(self)?;
        Ok(it)
    }
    fn by_user(&mut self, user: i64, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq(user))
            .filter(attachments::dsl::deleted_at.is_null())
            .order(attachments::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }

    fn count_by_user(&mut self, user: i64) -> Result<i64> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq(user))
            .count()
            .first(self)?;
        Ok(it)
    }
    fn delete(&mut self, id: i64) -> Result<()> {
        delete(
            attachment_resources::dsl::attachment_resources
                .filter(attachment_resources::dsl::attachment_id.eq(id)),
        )
        .execute(self)?;
        delete(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
    fn associate_(&mut self, id: i64, resource_type: &str, resource_id: i64) -> Result<()> {
        let cnt: i64 = attachment_resources::dsl::attachment_resources
            .filter(attachment_resources::dsl::attachment_id.eq(id))
            .filter(attachment_resources::dsl::resource_type.eq(resource_type))
            .filter(attachment_resources::dsl::resource_id.eq(resource_id))
            .count()
            .get_result(self)?;
        if cnt == 0 {
            insert_into(attachment_resources::dsl::attachment_resources)
                .values((
                    attachment_resources::dsl::attachment_id.eq(id),
                    attachment_resources::dsl::resource_type.eq(resource_type),
                    attachment_resources::dsl::resource_id.eq(resource_id),
                ))
                .execute(self)?;
        }
        Ok(())
    }
    fn dissociate_(&mut self, id: i64, resource_type: &str, resource_id: i64) -> Result<()> {
        delete(
            attachment_resources::dsl::attachment_resources
                .filter(attachment_resources::dsl::attachment_id.eq(id))
                .filter(attachment_resources::dsl::resource_type.eq(resource_type))
                .filter(attachment_resources::dsl::resource_id.eq(resource_id)),
        )
        .execute(self)?;
        Ok(())
    }
    fn clear_(&mut self, resource_type: &str, resource_id: i64) -> Result<()> {
        delete(
            attachment_resources::dsl::attachment_resources
                .filter(attachment_resources::dsl::resource_type.eq(resource_type))
                .filter(attachment_resources::dsl::resource_id.eq(resource_id)),
        )
        .execute(self)?;
        Ok(())
    }
    fn by_resource_(&mut self, resource_type: &str, resource_id: i64) -> Result<Vec<Item>> {
        let ids: Vec<i64> = attachment_resources::dsl::attachment_resources
            .select(attachment_resources::dsl::attachment_id)
            .filter(attachment_resources::dsl::resource_type.eq(resource_type))
            .filter(attachment_resources::dsl::resource_id.eq(resource_id))
            .order(attachment_resources::dsl::created_at.desc())
            .load(self)?;

        let items = attachments::dsl::attachments
            .filter(attachments::dsl::id.eq_any(ids))
            .load::<Item>(self)?;
        Ok(items)
    }
    fn associate<T>(&mut self, id: i64, resource_id: i64) -> Result<()> {
        self.associate_(id, type_name::<T>(), resource_id)
    }
    fn dissociate<T>(&mut self, id: i64, resource_id: i64) -> Result<()> {
        self.dissociate_(id, type_name::<T>(), resource_id)
    }
    fn clear<T>(&mut self, resource_id: i64) -> Result<()> {
        self.clear_(type_name::<T>(), resource_id)
    }
    fn by_resource<T>(&mut self, resource_id: i64) -> Result<Vec<Item>> {
        self.by_resource_(type_name::<T>(), resource_id)
    }
}
