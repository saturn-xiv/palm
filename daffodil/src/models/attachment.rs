use std::any::type_name;
use std::path::Path;
use std::string::ToString;

use chrono::{Datelike, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use mime::Mime;
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};
use uuid::Uuid;

use super::super::schema::{attachment_resources, attachments};

#[derive(Queryable, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub public: bool,
    pub bucket: String,
    pub object: String,
    pub title: String,
    pub size: i64,
    pub content_type: String,
    pub uploaded_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn bucket_by_year_month() -> String {
        let now = Utc::now();
        format!("{}-{:02}", now.year(), now.month())
    }
    pub fn object(title: &str) -> String {
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
                let ct = match it {
                    "png" => mime::IMAGE_PNG,
                    "svg" => mime::IMAGE_SVG,
                    "jpg" | "jpeg" => mime::IMAGE_JPEG,
                    "js" => mime::APPLICATION_JAVASCRIPT_UTF_8,
                    "css" => mime::TEXT_CSS_UTF_8,
                    "csv" => mime::TEXT_CSV_UTF_8,
                    "pdf" => mime::APPLICATION_PDF,
                    "txt" | "md" => mime::TEXT_PLAIN_UTF_8,
                    v => {
                        log::warn!("unknown file extension: {}", v);
                        mime::APPLICATION_OCTET_STREAM
                    }
                };
                return ct;
            }
        }
        log::warn!("unknown file type: {}", title);
        mime::APPLICATION_OCTET_STREAM
    }
}

#[derive(EnumString, EnumDisplay, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Public,
    Private,
}

pub trait Dao {
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn create(
        &mut self,
        user: i32,
        public: bool,
        bucket: &str,
        object: &str,
        info: (&str, &Mime, u64),
    ) -> Result<()>;
    fn set_title(&mut self, id: i32, title: &str) -> Result<()>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count(&mut self) -> Result<i64>;
    fn by_user(&mut self, user: i32, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_resource<T>(&mut self, resource_id: i32) -> Result<Vec<Item>>;
    fn by_resource_type<T>(&mut self) -> Result<Vec<Item>>;
    fn by_resource_(&mut self, resource_type: &str, resource_id: i32) -> Result<Vec<Item>>;
    fn by_resource_type_(&mut self, resource_type: &str) -> Result<Vec<Item>>;

    fn count_by_user(&mut self, user: i32) -> Result<i64>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;

    fn associate<T>(&mut self, id: i32, resource_id: i32) -> Result<()>;
    fn dissociate<T>(&mut self, id: i32, resource_id: i32) -> Result<()>;
    fn associate_(&mut self, id: i32, resource_type: &str, resource_id: i32) -> Result<()>;
    fn dissociate_(&mut self, id: i32, resource_type: &str, resource_id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i32,
        public: bool,
        bucket: &str,
        object: &str,
        (title, content_type, size): (&str, &Mime, u64),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();

        let content_type = content_type.to_string();
        insert_into(attachments::dsl::attachments)
            .values((
                attachments::dsl::user_id.eq(user),
                attachments::dsl::public.eq(public),
                attachments::dsl::bucket.eq(bucket),
                attachments::dsl::object.eq(object),
                attachments::dsl::title.eq(title),
                attachments::dsl::content_type.eq(content_type),
                attachments::dsl::size.eq(size as i64),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn set_title(&mut self, id: i32, title: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id)))
            .set((
                attachments::dsl::title.eq(title),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
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
    fn by_user(&mut self, user: i32, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq(user))
            .order(attachments::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_resource<T>(&mut self, resource_id: i32) -> Result<Vec<Item>> {
        self.by_resource_(type_name::<T>(), resource_id)
    }
    fn by_resource_type<T>(&mut self) -> Result<Vec<Item>> {
        self.by_resource_type_(type_name::<T>())
    }
    fn by_resource_(&mut self, resource_type: &str, resource_id: i32) -> Result<Vec<Item>> {
        let ids: Vec<i32> = attachment_resources::dsl::attachment_resources
            .select(attachment_resources::dsl::attachment_id)
            .filter(attachment_resources::dsl::resource_type.eq(resource_type))
            .filter(attachment_resources::dsl::resource_id.eq(resource_id))
            .order(attachment_resources::dsl::created_at.desc())
            .load(self)?;
        let items = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq_any(ids))
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_resource_type_(&mut self, resource_type: &str) -> Result<Vec<Item>> {
        let ids: Vec<i32> = attachment_resources::dsl::attachment_resources
            .select(attachment_resources::dsl::attachment_id)
            .filter(attachment_resources::dsl::resource_type.eq(resource_type))
            .order(attachment_resources::dsl::created_at.desc())
            .load(self)?;
        let items = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq_any(ids))
            .load::<Item>(self)?;
        Ok(items)
    }
    fn count_by_user(&mut self, user: i32) -> Result<i64> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq(user))
            .count()
            .first(self)?;
        Ok(it)
    }
    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = attachments::dsl::attachments.filter(attachments::dsl::id.eq(id));
        update(it)
            .set((
                attachments::dsl::deleted_at.eq(&Some(now)),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = attachments::dsl::attachments.filter(attachments::dsl::id.eq(id));
        update(it)
            .set((
                attachments::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn associate<T>(&mut self, id: i32, resource_id: i32) -> Result<()> {
        self.associate_(id, type_name::<T>(), resource_id)
    }
    fn dissociate<T>(&mut self, id: i32, resource_id: i32) -> Result<()> {
        self.dissociate_(id, type_name::<T>(), resource_id)
    }
    fn associate_(&mut self, id: i32, resource_type: &str, resource_id: i32) -> Result<()> {
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
    fn dissociate_(&mut self, id: i32, resource_type: &str, resource_id: i32) -> Result<()> {
        delete(
            attachment_resources::dsl::attachment_resources
                .filter(attachment_resources::dsl::attachment_id.eq(id))
                .filter(attachment_resources::dsl::resource_type.eq(resource_type))
                .filter(attachment_resources::dsl::resource_id.eq(resource_id)),
        )
        .execute(self)?;
        Ok(())
    }
}
