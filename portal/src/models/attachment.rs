use std::path::Path;

use chrono::{Duration, NaiveDateTime, Utc};
use data_encoding::BASE64URL_NOPAD;
use diesel::{insert_into, prelude::*, update};
use flatbuffers::{FlatBufferBuilder, ForwardsUOffset, Vector};
use hyacinth::schema::attachments;
use hyper::StatusCode;
use mime_guess::Mime;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use super::super::{HttpError, Jwt, Result, graphql::CurrentUser, orm::postgresql::Connection};

#[derive(Debug, Clone, Default, Queryable, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub bucket: String,
    pub object: String,
    pub title: String,
    pub size: Option<i64>,
    pub content_type: String,
    pub public: bool,
    pub uploaded_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Item {
    pub async fn url<J: Jwt>(&self, jwt: &J, ttl: Duration) -> Result<String> {
        let uid = self.uid();
        let token = if self.public {
            "_".to_string()
        } else {
            jwt.sign(
                CurrentUser::ISSUER,
                "",
                vec![Self::AUDIENCE],
                ttl,
                None::<JsonValue>,
            )
            .await?
        };
        Ok(format!("/attachments/{token}/{uid}"))
    }
    pub fn content_type<P: AsRef<Path>>(file: P) -> Mime {
        mime_guess::from_path(file).first_or_octet_stream()
    }
    pub fn online(&self) -> bool {
        if let Some(x) = Path::new(&self.title).extension().and_then(|x| x.to_str()) {
            return vec![
                "txt", "html", "htm", "css", "js", "json", "xml", "png", "svg", "jpb", "jpeg",
                "bmp", "pdf",
            ]
            .contains(&x);
        }
        false
    }
    pub fn uid(&self) -> String {
        let mut builder = FlatBufferBuilder::new();
        let mut offsets = Vec::new();
        {
            let it = builder.create_string(&self.bucket);
            offsets.push(it);
        };
        {
            let it = builder.create_string(&self.object);
            offsets.push(it);
        };
        let root = builder.create_vector(&offsets);
        builder.finish(root, None);
        let buf = builder.finished_data();
        BASE64URL_NOPAD.encode(buf)
    }

    pub fn from_uid(uid: &str) -> Result<(String, String)> {
        let buf = BASE64URL_NOPAD.decode(uid.as_bytes())?;
        let tmp = flatbuffers::root::<Vector<ForwardsUOffset<&str>>>(&buf)?;
        if tmp.len() != 2 {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("invalid attachment uid".to_string()),
            )));
        }
        Ok((tmp.get(0).to_string(), tmp.get(1).to_string()))
    }

    pub const AUDIENCE: &str = "attachment.show";
}

pub trait Dao {
    fn count_by_user(&mut self, user: i64) -> Result<i64>;
    fn count(&mut self) -> Result<i64>;
    fn index(&mut self, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn index_by_user(&mut self, user: i64, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn by_bucket_and_object(&mut self, bucket: &str, object: &str) -> Result<Item>;
    fn delete(&mut self, id: i64) -> Result<()>;
    fn create(&mut self, user: i64, file: (&str, &Mime), s3: (&str, &str, bool)) -> Result<()>;
    fn set_uploaded_at(&mut self, id: i64, size: usize) -> Result<()>;
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
        (title, content_type): (&str, &Mime),
        (bucket, object, public): (&str, &str, bool),
    ) -> Result<()> {
        let content_type = content_type.to_string();
        let now = Utc::now().naive_utc();
        insert_into(attachments::dsl::attachments)
            .values((
                attachments::dsl::user_id.eq(user),
                attachments::dsl::title.eq(title),
                attachments::dsl::content_type.eq(&content_type),
                attachments::dsl::bucket.eq(bucket),
                attachments::dsl::object.eq(object),
                attachments::dsl::public.eq(public),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_uploaded_at(&mut self, id: i64, size: usize) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = attachments::dsl::attachments.filter(attachments::dsl::id.eq(id));
        update(it)
            .set((
                attachments::dsl::size.eq(size as i64),
                attachments::dsl::uploaded_at.eq(&now),
                attachments::dsl::version.eq(attachments::dsl::version + 1),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
