use std::{fs::read_to_string, path::Path, result::Result as StdResult};

use askama::Template;
use futures_util::StreamExt;
use hyper::StatusCode;
use minio::s3::{
    MinioClient,
    builders::ObjectContent,
    creds::StaticProvider,
    error::Error as MinioError,
    http::BaseUrl,
    lifecycle_config::{LifecycleConfig, LifecycleRule},
    response::{BucketExistsResponse, CreateBucketResponse},
    response_traits::{HasBucket, HasRegion, HasVersion},
    types::{S3Api, ToStream},
};
use serde::{Deserialize, Serialize};
use serde_json::from_str as json_from_str;
use url::Url;
use uuid::Uuid;

use super::{HttpError, Result};

type MinioResult<T> = StdResult<T, MinioError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub url: String,
    pub access_key: String,
    pub secret_key: String,
    pub api: String,
    pub path: String,
}

impl Config {
    pub fn open<P: AsRef<Path>, T: ToString>(file: P, namespace: Option<T>) -> Result<Client> {
        let it: Self = json_from_str(&read_to_string(file)?)?;
        let cfg = Node {
            endpoint: {
                let u = Url::parse(&it.url)?;
                let h = u.host_str().ok_or_else(|| {
                    Box::new(HttpError(
                        StatusCode::BAD_REQUEST,
                        Some("invalid host".to_string()),
                    ))
                })?;
                match u.port() {
                    Some(p) => format!("{}://{}:{}", u.scheme(), h, p),
                    None => format!("{}://{}", u.scheme(), h),
                }
            },
            access_key: it.access_key.clone(),
            secret_key: it.secret_key,
            namespace: namespace.map(|x| x.to_string()),
        };
        cfg.open()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(default = "node_default_endpoint")]
    pub endpoint: String,
    #[serde(rename = "access-key")]
    pub access_key: String,
    #[serde(rename = "secret-key")]
    pub secret_key: String,
    #[serde(default)]
    pub namespace: Option<String>,
}

fn node_default_endpoint() -> String {
    "http://127.0.0.1:9000".to_string()
}

impl Node {
    pub fn open(&self) -> Result<Client> {
        log::info!("open minio {}", self.endpoint);
        let url = self.endpoint.parse::<BaseUrl>()?;
        let provider = StaticProvider::new(&self.access_key, &self.secret_key, None);
        let client = MinioClient::new(url, Some(provider), None, None)?;
        Ok(Client {
            s3: client,
            namespace: self.namespace.clone(),
        })
    }
}

pub struct Client {
    namespace: Option<String>,
    s3: MinioClient,
}

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
impl Client {
    pub async fn delete_object(&self, bucket: &str, object: &str) -> MinioResult<()> {
        log::warn!("delete object {}/{}", bucket, object);
        let res = self
            .s3
            .delete_object(bucket, object)?
            .build()
            .send()
            .await?;
        log::debug!(
            "the object is deleted. The delete marker has version '{:?}'",
            res.version_id()
        );
        Ok(())
    }
    pub async fn delete_bucket(&self, bucket: &str) -> MinioResult<()> {
        log::warn!("delete bucket {}", bucket);
        let res = self.s3.delete_bucket(bucket)?.build().send().await?;
        log::debug!(
            "bucket {:?} in region {} was deleted",
            res.bucket().map(|x| x.as_str()),
            res.region().as_str()
        );
        Ok(())
    }
    pub async fn list_objects(&self, bucket: &str) -> MinioResult<Vec<String>> {
        let mut res = self
            .s3
            .list_objects(bucket)?
            .recursive(true)
            .use_api_v1(false)
            .include_versions(true)
            .build()
            .to_stream()
            .await;
        let mut items = Vec::new();
        while let Some(entry) = res.next().await {
            let entry = entry?;
            for it in entry.contents.iter() {
                items.push(it.name.clone());
            }
        }
        Ok(items)
    }
    pub async fn list_buckets(&self) -> MinioResult<Vec<String>> {
        let res = self.s3.list_buckets().build().send().await?;
        let mut items = Vec::new();
        for it in res.buckets()?.iter() {
            items.push(it.name.as_str().to_string());
        }
        Ok(items)
    }
    pub async fn bucket_exists(&self, name: &str) -> MinioResult<bool> {
        let res: BucketExistsResponse = self.s3.bucket_exists(name)?.build().send().await?;
        Ok(res.exists())
    }

    pub async fn create_bucket(
        &self,
        name: &str,
        public: bool,
        expire_after_days: Option<usize>,
    ) -> Result<String> {
        log::info!("create bucket {}", name);
        let bucket = match self.namespace {
            Some(ref it) => format!("{}.{}", it, name),
            None => name.to_string(),
        };

        let res: CreateBucketResponse = self.s3.create_bucket(&bucket)?.build().send().await?;
        log::debug!(
            "made bucket {:?} in region {}",
            res.bucket().map(|x| x.as_str()),
            res.region().as_str()
        );

        if public {
            log::info!("set anonymous read access to bucket {}", bucket);
            let policy = {
                let it = PublicBucketPolicyTemplate { name: &bucket };
                it.render()?
            };
            self.s3
                .put_bucket_policy(&bucket)?
                .config(policy)
                .build()
                .send()
                .await?;
        }
        if let Some(expire_after_days) = expire_after_days
            && expire_after_days > 0
        {
            log::info!("set lifecycle rule({} days)", expire_after_days);
            let rules: Vec<LifecycleRule> = vec![LifecycleRule {
                id: format!("expire-after-{}-days", expire_after_days),
                expiration_days: Some(expire_after_days as u32),
                status: true,
                ..Default::default()
            }];

            self.s3
                .put_bucket_lifecycle(&bucket)?
                .life_cycle_config(LifecycleConfig { rules })
                .build()
                .send()
                .await?;
        }

        Ok(bucket)
    }

    pub async fn upload<P: AsRef<Path>>(
        &self,
        bucket: &str,
        object: &str,
        file: P,
    ) -> MinioResult<()> {
        let file = file.as_ref();
        log::info!("upload file {} to {}/{}", file.display(), bucket, object);

        let content = ObjectContent::from(file);
        self.s3
            .put_object_content(bucket, object, content)?
            .build()
            .send()
            .await?;
        Ok(())
    }

    pub fn object<P: AsRef<Path>>(file: P) -> String {
        let uid = Uuid::new_v4().to_string();
        let file = file.as_ref();
        match file.extension() {
            Some(ext) => format!("{}.{}", uid, ext.display()),
            None => uid,
        }
    }
}

#[derive(Template)]
#[template(path = "minio/bucket-anonymous-read.json", escape = "none")]
struct PublicBucketPolicyTemplate<'a> {
    name: &'a str,
}
