use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use hyacinth::schema::attachments;
use serde::{Deserialize, Serialize};

use super::super::{Result, orm::postgresql::Connection};

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub bucket: String,
    pub object: String,
    pub title: String,
    pub size: i64,
    pub content_type: String,
    pub public: bool,
    pub uploaded_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn count_by_user(&mut self, user: i64) -> Result<i64>;
    fn count(&mut self) -> Result<i64>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn index_by_user(&mut self, user: i64, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_bucket_and_object(&mut self, bucket: &str, object: &str) -> Result<Item>;
    fn delete(&mut self, id: i64) -> Result<()>;
    fn create(&mut self, user: i64, file: (&str, &str, i64), s3: (&str, &str, bool)) -> Result<()>;
    fn set_uploaded_at(&mut self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn count_by_user(&mut self, user: i64) -> Result<i64> {
        let it: i64 = attachments::dsl::attachments
            .count()
            .filter(attachments::dsl::user_id.eq(user))
            .get_result(self)?;
        Ok(it)
    }
    fn count(&mut self) -> Result<i64> {
        let cnt: i64 = attachments::dsl::attachments.count().get_result(self)?;
        Ok(cnt)
    }
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .order(attachments::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn index_by_user(&mut self, user: i64, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq(user))
            .order(attachments::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_bucket_and_object(&mut self, bucket: &str, object: &str) -> Result<Item> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::bucket.eq(&bucket))
            .filter(attachments::dsl::object.eq(object))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn delete(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = attachments::dsl::attachments.filter(attachments::dsl::id.eq(id));
        update(it)
            .set((
                attachments::dsl::version.eq(attachments::dsl::version + 1),
                attachments::dsl::deleted_at.eq(&now),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn create(
        &mut self,
        user: i64,
        (title, content_type, size): (&str, &str, i64),
        (bucket, object, public): (&str, &str, bool),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(attachments::dsl::attachments)
            .values((
                attachments::dsl::user_id.eq(user),
                attachments::dsl::title.eq(title),
                attachments::dsl::content_type.eq(content_type),
                attachments::dsl::size.eq(size),
                attachments::dsl::bucket.eq(bucket),
                attachments::dsl::object.eq(object),
                attachments::dsl::public.eq(public),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_uploaded_at(&mut self, id: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = attachments::dsl::attachments.filter(attachments::dsl::id.eq(id));
        update(it)
            .set((
                attachments::dsl::version.eq(attachments::dsl::version + 1),
                attachments::dsl::uploaded_at.eq(&now),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
