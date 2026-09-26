use std::path::Path;

use aws_config::{BehaviorVersion, defaults as aws_config_defaults};
use aws_credential_types::Credentials;
use aws_sdk_s3::{
    Client as S3Client,
    presigning::PresigningConfig,
    primitives::{ByteStream, DateTime},
    types::{
        BucketLifecycleConfiguration, BucketLocationConstraint, CreateBucketConfiguration,
        ExpirationStatus, Grantee, LifecycleExpiration, LifecycleRule, LifecycleRuleFilter,
        ObjectCannedAcl, Permission, Tag, Tagging, Type,
    },
};
use chrono::Duration;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use super::super::{HttpError, Result};

// https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_s3_code_examples.html#get_started
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(rename = "access-key-id")]
    pub access_key_id: String,
    #[serde(rename = "secret-access-key")]
    pub secret_access_key: String,
    pub region: String,
}

impl Config {
    pub async fn open(&self) -> Result<Client> {
        let credentials =
            Credentials::from_keys(&self.access_key_id, &self.secret_access_key, None);
        let config = aws_config_defaults(BehaviorVersion::latest())
            .credentials_provider(credentials)
            .load()
            .await;
        Ok(Client {
            s3: S3Client::new(&config),
            region: self.region.clone(),
        })
    }
}

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
pub struct Client {
    s3: S3Client,
    region: String,
}
impl Client {
    pub async fn get_object(
        &self,
        bucket: &str,
        object: &str,
        version_id: Option<&str>,
    ) -> Result<String> {
        if !self
            .can_public_read_object(bucket, object, version_id)
            .await?
        {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }

        let url = format!(
            "https://s3.{}.amazonaws.com/{}/{}",
            self.region, bucket, object
        );
        if let Some(version_id) = version_id {
            return Ok(format!("{url}?versionId={version_id}"));
        }
        Ok(url)
    }
    async fn can_public_read_object(
        &self,
        bucket: &str,
        object: &str,
        version_id: Option<&str>,
    ) -> Result<bool> {
        let res = {
            let mut it = self.s3.get_object_acl().bucket(bucket).key(object);
            if let Some(version_id) = version_id {
                it = it.version_id(version_id);
            }
            it.send().await?
        };
        if let Some(grants) = res.grants {
            for grant in grants {
                let is_all_users = match grant.grantee {
                    Some(Grantee {
                        r#type: Type::Group,
                        uri: Some(ref uri),
                        ..
                    }) => uri == "http://acs.amazonaws.com/groups/global/AllUsers",
                    _ => false,
                };

                let is_read_permission = matches!(
                    grant.permission(),
                    Some(Permission::Read) | Some(Permission::FullControl)
                );

                if is_all_users && is_read_permission {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
    pub async fn get_presigned_url(
        &self,
        bucket: &str,
        object: &str,
        version_id: Option<&str>,
        expires_in: Duration,
    ) -> Result<String> {
        if self
            .can_public_read_object(bucket, object, version_id)
            .await?
        {
            return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
        }
        let mut it = self.s3.get_object().bucket(bucket).key(object);
        if let Some(version_id) = version_id {
            it = it.version_id(version_id);
        }
        let res = it
            .presigned(
                PresigningConfig::builder()
                    .expires_in(expires_in.to_std()?)
                    .build()?,
            )
            .await?;
        Ok(res.uri().to_string())
    }
    pub async fn list_object_versions(
        &self,
        bucket: &str,
        object: &str,
    ) -> Result<Vec<(Option<String>, Option<DateTime>)>> {
        let res = self
            .s3
            .list_object_versions()
            .bucket(bucket)
            .prefix(object)
            .send()
            .await?;
        let mut items = Vec::new();
        if let Some(ref versions) = res.versions {
            for version in versions.iter() {
                items.push((
                    version.version_id().map(|x| x.to_string()),
                    version.last_modified().copied(),
                ));
            }
        }
        Ok(items)
    }
    pub async fn delete_object(&self, bucket: &str, object: &str) -> Result<()> {
        self.s3
            .delete_object()
            .bucket(bucket)
            .key(object)
            .send()
            .await?;
        Ok(())
    }
    pub async fn put_object<P: AsRef<Path>>(
        &self,
        file: P,
        bucket: &str,
        object: &str,
        public: bool,
        auto_expired: bool,
    ) -> Result<()> {
        let file = file.as_ref();
        log::info!("upload {} to {}/{}", file.display(), bucket, object);
        let body = ByteStream::from_path(file).await?;
        {
            let mut it = self.s3.put_object().bucket(bucket).key(object).body(body);
            if public {
                it = it.acl(ObjectCannedAcl::PublicRead);
            }
            it.send().await?;
        }

        if auto_expired {
            self.s3
                .put_object_tagging()
                .bucket(bucket)
                .key(object)
                .tagging(Tagging::builder().tag_set(Self::ttl_tag()?).build()?)
                .send()
                .await?;
        }

        Ok(())
    }

    fn ttl_tag() -> Result<Tag> {
        let it = Tag::builder().key("ttl").value("yes").build()?;
        Ok(it)
    }
    pub async fn create_bucket(&self, name: &str, expired_in_days: Option<i32>) -> Result<()> {
        let constraint = BucketLocationConstraint::from(self.region.as_str());
        let config = CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();
        log::info!("create bucket {name}");
        self.s3
            .create_bucket()
            .create_bucket_configuration(config)
            .bucket(name)
            .send()
            .await?;
        if let Some(days) = expired_in_days {
            let id = format!("DeleteTemporaryObjectsAfter{days}Days");
            log::info!("set lifecycle {id} for bucket {name}");
            self.s3
                .put_bucket_lifecycle_configuration()
                .bucket(name)
                .lifecycle_configuration(
                    BucketLifecycleConfiguration::builder()
                        .rules(
                            LifecycleRule::builder()
                                .id(&id)
                                .status(ExpirationStatus::Enabled)
                                .filter(
                                    LifecycleRuleFilter::builder().tag(Self::ttl_tag()?).build(),
                                )
                                .expiration(LifecycleExpiration::builder().days(days).build())
                                .build()?,
                        )
                        .build()?,
                )
                .send()
                .await?;
        }

        Ok(())
    }

    pub async fn bucket_exists(&self, name: &str) -> Result<()> {
        self.s3.head_bucket().bucket(name).send().await?;
        Ok(())
    }
}

/*

use std::{fs::read_to_string, path::Path, result::Result as StdResult};

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
use serde_json::from_str as json_from_str;
use url::Url;
use uuid::Uuid;

use super::{HttpError, Result};

type MinioResult<T> = StdResult<T, MinioError>;


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



 */
