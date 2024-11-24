pub mod v1 {
    tonic::include_proto!("palm.s3.v1");
}

use std::fmt;
use std::path::Path;
use std::str::FromStr;

use askama::Template;
use chrono::Duration;
use data_encoding::BASE32_NOPAD;
use hyper::Method;
use minio::s3::{
    args::{
        BucketExistsArgs, GetPresignedObjectUrlArgs, MakeBucketArgs, SetBucketLifecycleArgs,
        SetBucketPolicyArgs,
    },
    builders::ObjectContent,
    creds::StaticProvider,
    http::BaseUrl,
    types::{Filter, LifecycleConfig, LifecycleRule, S3Api},
    Client as MinioClient, ClientBuilder,
};
use prost::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(rename = "base-url")]
    pub base_url: String,
    #[serde(rename = "access-key")]
    pub access_key: String,
    #[serde(rename = "secret-key")]
    pub secret_key: String,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:9000".to_string(),
            access_key: "change-me".to_string(),
            secret_key: "change-me".to_string(),
        }
    }
}

impl Config {
    pub fn open(&self) -> Result<Client> {
        let base_url = self.base_url.parse::<BaseUrl>()?;
        let client = ClientBuilder::new(base_url)
            .provider(Some(Box::new(StaticProvider::new(
                &self.access_key,
                &self.secret_key,
                None,
            ))))
            .build()?;
        Ok(Client {
            client,
            base_url: self.base_url.clone(),
        })
    }
}

pub struct Client {
    client: MinioClient,
    base_url: String,
}

impl Client {
    pub async fn list_buckets(&self) -> Result<Vec<String>> {
        let items = self.client.list_buckets().send().await?;
        Ok(items.buckets.into_iter().map(|x| x.name).collect())
    }
    // pub async fn list_objects(&self, bucket: &str) -> Result<Vec<String>> {
    //     let items = self.client.list_objects(bucket).send().await?;
    //     Ok(items.objects.into_iter().map(|x| x.name).collect())
    // }
    pub async fn remove_object(&self, bucket: &str, object: &str) -> Result<()> {
        log::warn!("remove {}/{}", bucket, object);
        self.client.remove_object(bucket, object).send().await?;
        Ok(())
    }
    pub async fn get_object_url(
        &self,
        bucket: &str,
        object: &str,
        ttl: Option<Duration>,
    ) -> Result<String> {
        {
            let it = bucket.parse::<v1::Bucket>()?;
            if it.public {
                return Ok(format!("{}/{}/{}", self.base_url, bucket, object));
            }
        }
        let ttl = {
            let max = Duration::days(7);
            let min = Duration::minutes(5);
            match ttl {
                Some(it) => {
                    if it > max {
                        max
                    } else if it < min {
                        min
                    } else {
                        it
                    }
                }
                None => max,
            }
        };

        let args = {
            let mut it = GetPresignedObjectUrlArgs::new(bucket, object, Method::GET)?;
            it.expiry_seconds = Some(ttl.num_seconds() as u32);
            it
        };
        let res = self.client.get_presigned_object_url(&args).await?;
        Ok(res.url)
    }

    fn file_name<P: AsRef<Path>>(file: P) -> String {
        let it = Uuid::new_v4().to_string();
        let file = file.as_ref();
        if let Some(ext) = file.extension() {
            if let Some(ext) = ext.to_str() {
                return format!("{it}.{ext}");
            }
        }
        it
    }
    pub async fn upload_object<P: AsRef<Path>>(
        &self,
        bucket: &str,
        title: &str,
        file: P,
    ) -> Result<String> {
        let file = file.as_ref();
        let object = Self::file_name(title);
        {
            log::info!("upload {} to {}/{}", file.display(), bucket, object);
            let content = ObjectContent::from(file);
            self.client
                .put_object_content(bucket, &object, content)
                .send()
                .await?;
        }

        Ok(object)
    }
    pub async fn create_bucket(
        &self,
        name: &str,
        public: bool,
        expiration_days: Option<usize>,
    ) -> Result<String> {
        let name = {
            let it = v1::Bucket {
                name: name.to_string(),
                public,
                expiration_days: expiration_days.map(|x| x as u32),
            };
            it.to_string()
        };
        let found = self
            .client
            .bucket_exists(&BucketExistsArgs::new(&name)?)
            .await?;
        if !found {
            log::info!("create bucket {name}");
            self.client
                .make_bucket(&MakeBucketArgs::new(&name)?)
                .await?;
            if public {
                let policy = {
                    let it = BucketPolicy {
                        // https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_version.html
                        version: "2012-10-17",
                        name: &name,
                    };
                    it.render()?
                };
                log::info!("set bucket({name}) policy({policy})");
                self.client
                    .set_bucket_policy(&SetBucketPolicyArgs::new(&name, &policy)?)
                    .await?;
            }
            if expiration_days.is_some() {
                log::info!(
                    "set bucket({}) expiration days({:?})",
                    name,
                    expiration_days
                );

                self.client
                    .set_bucket_lifecycle(&SetBucketLifecycleArgs::new(
                        &name,
                        &LifecycleConfig {
                            rules: vec![LifecycleRule {
                                abort_incomplete_multipart_upload_days_after_initiation: None,
                                expiration_date: None,
                                expiration_days,
                                expiration_expired_object_delete_marker: None,
                                filter: Filter {
                                    and_operator: None,
                                    prefix: None,
                                    tag: None,
                                },
                                id: Uuid::new_v4().to_string(),
                                noncurrent_version_expiration_noncurrent_days: None,
                                noncurrent_version_transition_noncurrent_days: None,
                                noncurrent_version_transition_storage_class: None,
                                status: true,
                                transition_date: None,
                                transition_days: None,
                                transition_storage_class: None,
                            }],
                        },
                    )?)
                    .await?;
            }
        }
        Ok(name)
    }
}

#[derive(Template)]
#[template(path = "minio/bucket-policy.json", escape = "none")]
pub struct BucketPolicy<'a> {
    pub name: &'a str,
    pub version: &'a str,
}

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
impl fmt::Display for v1::Bucket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = {
            let mut buf = Vec::new();
            self.encode(&mut buf).map_err(|e| {
                log::error!("{}", e);
                fmt::Error
            })?;
            // bs58::encode(&buf).into_string()
            BASE32_NOPAD.encode(&buf).to_lowercase()
        };
        write!(f, "{}", s)
    }
}

impl FromStr for v1::Bucket {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // let buf = bs58::decode(s).into_vec()?;
        let buf = {
            let it = s.to_uppercase();
            BASE32_NOPAD.decode(it.as_bytes())?
        };
        let it = Self::decode(&buf[..])?;
        Ok(it)
    }
}
