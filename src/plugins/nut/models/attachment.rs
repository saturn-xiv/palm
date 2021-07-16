use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use mime::Mime;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::{
    CreateBucketRequest, DeleteObjectRequest, GetBucketLocationRequest, HeadBucketRequest,
    PutObjectRequest, S3Client, S3,
};
use serde::{Deserialize, Serialize};

use super::super::super::super::{
    crypto::Aes,
    orm::postgresql::{Connection, PooledConnection},
    settings::Dao as SettingDao,
    Result,
};
use super::schema::attachments;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Config {
    S3(AwsS3),
    Fs(FileSystem),
}
impl Config {
    pub const KEY: &'static str = "file.storage";
}

// pub trait Provider {
//     async fn store(&self, name: &str, payload: &[u8]) -> Result<String>;
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AwsS3 {
    pub region: String,
    pub endpoint: Option<String>,
    pub access_key: String,
    pub secret_key: String,
}

impl AwsS3 {
    fn open(&self) -> Result<S3Client> {
        let it = S3Client::new_with(
            HttpClient::new()?,
            StaticProvider::new(self.access_key.clone(), self.access_key.clone(), None, None),
            match self.endpoint {
                Some(ref v) => Region::Custom {
                    name: self.region.clone(),
                    endpoint: v.to_string(),
                },
                None => self.region.parse()?,
            },
        );
        Ok(it)
    }

    pub async fn put(&self, bucket: &str, name: &str, body: &[u8]) -> Result<()> {
        let client = self.open()?;
        if client
            .head_bucket(HeadBucketRequest {
                bucket: bucket.to_string(),
                ..Default::default()
            })
            .await
            .is_err()
        {
            client
                .create_bucket(CreateBucketRequest {
                    bucket: bucket.to_string(),
                    ..Default::default()
                })
                .await?;
        }

        let body = body.to_owned();
        client
            .put_object(PutObjectRequest {
                key: name.to_string(),
                body: Some(body.into()),
                ..Default::default()
            })
            .await?;
        Ok(())
    }

    // https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region
    pub async fn get(&self, bucket: &str, name: &str) -> Result<String> {
        if let Some(ref endpoint) = self.endpoint {
            return Ok(format!("{}/{}/{}", endpoint, bucket, name));
        }
        let client = self.open()?;
        let val = client
            .get_bucket_location(GetBucketLocationRequest {
                bucket: bucket.to_string(),
                ..Default::default()
            })
            .await?;
        Ok(format!(
            "https://s3-{}.amazonaws.com/{}/{}",
            val.location_constraint.unwrap_or_else(|| "".to_string()),
            bucket,
            name
        ))
    }

    pub async fn delete(&self, bucket: &str, name: &str) -> Result<()> {
        let client = self.open()?;
        client
            .delete_object(DeleteObjectRequest {
                bucket: bucket.to_string(),
                key: name.to_string(),
                ..Default::default()
            })
            .await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileSystem {
    pub root: PathBuf,
}

impl Default for FileSystem {
    fn default() -> Self {
        Self {
            root: Path::new("tmp").join("uploads"),
        }
    }
}

impl FileSystem {
    pub async fn store(&self, bucket: &str, name: &str, payload: &[u8]) -> Result<String> {
        let root = self.root.join(bucket);
        create_dir_all(bucket)?;
        {
            let file = root.join(name);
            info!("save to file {}", file.display());
            let mut file = File::create(file)?;
            file.write_all(payload)?;
        }
        Ok(format!("/uploads/{}/{}", bucket, name))
    }
}

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub size: i32,
    pub content_type: String,
    pub url: String,
    pub version: i32,
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
    pub async fn store(
        db: PooledConnection,
        aes: &Aes,
        user: i32,
        title: &str,
        payload: &[u8],
    ) -> Result<()> {
        let (bucket, name, content_type) = Self::detect(title);
        let cfg: Config = {
            let db = db.deref();
            SettingDao::get(db, aes, Config::KEY)?
        };
        let url = match cfg {
            Config::S3(it) => {
                it.put(&bucket, &name, payload).await?;
                it.get(&bucket, &name).await?
            }
            Config::Fs(it) => it.store(&bucket, &name, payload).await?,
        };
        let db = db.deref();
        Dao::create(db, user, title, &content_type, &url, payload.len() as i32)?;
        Ok(())
    }
}

pub trait Dao {
    fn by_id(&self, id: i32) -> Result<Item>;
    fn create(
        &self,
        user: i32,
        title: &str,
        content_type: &Mime,
        url: &str,
        size: i32,
    ) -> Result<()>;
    fn update(&self, id: i32, title: &str, content_type: &Mime, url: &str, size: i32)
        -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn by_user(&self, user: i32) -> Result<Vec<Item>>;
    fn delete(&self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: i32) -> Result<Item> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &self,
        user: i32,
        title: &str,
        content_type: &Mime,
        url: &str,
        size: i32,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let content_type = content_type.to_string();
        insert_into(attachments::dsl::attachments)
            .values((
                attachments::dsl::user_id.eq(user),
                attachments::dsl::title.eq(title),
                attachments::dsl::content_type.eq(content_type),
                attachments::dsl::url.eq(url),
                attachments::dsl::size.eq(size),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(
        &self,
        id: i32,
        title: &str,
        content_type: &Mime,
        url: &str,
        size: i32,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let content_type = content_type.to_string();
        update(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id)))
            .set((
                attachments::dsl::title.eq(title),
                attachments::dsl::content_type.eq(content_type),
                attachments::dsl::url.eq(url),
                attachments::dsl::size.eq(size),
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

    fn by_user(&self, user: i32) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq(user))
            .order(attachments::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: i32) -> Result<()> {
        delete(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
