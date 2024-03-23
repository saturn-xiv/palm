use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use ::minio::s3::{
    args::{
        BucketExistsArgs, GetPresignedObjectUrlArgs, ListBucketsArgs, ListObjectsV2Args,
        MakeBucketArgs, PutObjectArgs, RemoveBucketArgs, RemoveObjectArgs, SetBucketLifecycleArgs,
        SetBucketPolicyArgs,
    },
    client::{Client as MinioClient, ClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
    types::{Filter, LifecycleConfig, LifecycleRule},
    utils::Multimap,
};
use askama::Template;
use chrono::{DateTime, Datelike, Duration, Utc};
use log::{debug, info};
use mime::Mime;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::Result;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub endpoint: String,
    #[serde(rename = "access-key")]
    pub access_key: String,
    #[serde(rename = "secret-key")]
    pub secret_key: String,
    pub bucket: String,
}

#[derive(Template)]
#[template(path = "minio/service.conf", escape = "none")]
pub struct SystemdConfig<'a> {
    pub domain: &'a str,
    pub port: u16,
    pub console_port: u16,
    pub user: &'a str,
    pub password: &'a str,
}

impl SystemdConfig<'_> {
    pub fn write<P: AsRef<Path>>(&self, file: P) -> Result<()> {
        let file = file.as_ref();
        info!("generate file {}", file.display());
        let tpl = self.render()?;
        let mut fd = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(file)?;
        fd.write_all(tpl.as_bytes())?;
        Ok(())
    }
}

#[derive(Template)]
#[template(path = "minio/nginx.conf", escape = "none")]
pub struct NginxConfig<'a> {
    pub domain: &'a str,
    pub nodes: &'a [Node<'a>],
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node<'a> {
    pub host: &'a str,
    pub port: u16,
    pub console_port: u16,
}

impl NginxConfig<'_> {
    pub fn write<P: AsRef<Path>>(&self, file: P) -> Result<()> {
        let file = file.as_ref();
        info!("generate file {}", file.display());
        let tpl = self.render()?;
        let mut fd = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(file)?;
        fd.write_all(tpl.as_bytes())?;
        Ok(())
    }
}

impl Config {
    pub fn open(&self) -> Result<Client> {
        let cred = StaticProvider::new(&self.access_key, &self.secret_key, None);
        let base_url: BaseUrl = self.endpoint.parse()?;

        let connection = Connection {
            client: ClientBuilder::new(base_url.clone())
                .provider(Some(Box::new(cred)))
                .ignore_cert_check(Some(true))
                .build()?,
            base_url,
        };

        Ok(Client {
            connection,
            bucket: self.bucket.clone(),
        })
    }
}

pub struct Client {
    pub connection: Connection,
    bucket: String,
}

impl Client {
    pub async fn bucket(&self, public: bool, expiration_days: Option<usize>) -> Result<String> {
        let name = format!(
            "{}.{}.{}{}",
            self.bucket,
            Utc::now().year(),
            if public { "o" } else { "p" },
            expiration_days
                .map(|x| format!(".{x}"))
                .unwrap_or("".to_string())
        );
        if !self.connection.bucket_exists(&name).await? {
            self.connection
                .create_bucket(&name, public, expiration_days)
                .await?;
        }
        Ok(name)
    }
}

pub struct Connection {
    base_url: BaseUrl,
    client: MinioClient,
}
impl Connection {
    pub async fn bucket_exists(&self, name: &str) -> Result<bool> {
        let ok = self
            .client
            .bucket_exists(&BucketExistsArgs::new(name)?)
            .await?;
        Ok(ok)
    }
    pub async fn create_bucket(
        &self,
        name: &str,
        public: bool,
        expiration_days: Option<usize>,
    ) -> Result<()> {
        debug!("create bucket {}", name);
        self.client.make_bucket(&MakeBucketArgs::new(name)?).await?;
        if public {
            debug!("Set bucket {} to public", name);
            let policy = {
                let now = Utc::now().date_naive().to_string();
                let it = BucketPublicPolicy {
                    version: &now,
                    name,
                };
                it.render()?
            };
            debug!("{policy}");
            self.client
                .set_bucket_policy(&SetBucketPolicyArgs::new(name, &policy)?)
                .await?;
        }
        if expiration_days.is_some() {
            self.client
                .set_bucket_lifecycle(&SetBucketLifecycleArgs::new(
                    name,
                    &LifecycleConfig {
                        rules: vec![LifecycleRule {
                            id: "expire-bucket".to_string(),
                            status: true,
                            expiration_days,
                            expiration_date: None,
                            filter: Filter {
                                and_operator: None,
                                prefix: None,
                                tag: None,
                            },
                            abort_incomplete_multipart_upload_days_after_initiation: None,
                            expiration_expired_object_delete_marker: None,
                            noncurrent_version_expiration_noncurrent_days: None,
                            noncurrent_version_transition_noncurrent_days: None,
                            noncurrent_version_transition_storage_class: None,
                            transition_date: None,
                            transition_days: None,
                            transition_storage_class: None,
                        }],
                    },
                )?)
                .await?;
        }
        Ok(())
    }
    pub async fn delete_bucket(&self, name: &str) -> Result<()> {
        self.client
            .remove_bucket(&RemoveBucketArgs::new(name)?)
            .await?;
        Ok(())
    }
    pub async fn list_buckets(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        let res = self.client.list_buckets(&ListBucketsArgs::new()).await?;
        let items = res
            .buckets
            .iter()
            .map(|x| (x.name.clone(), x.creation_date))
            .collect();
        Ok(items)
    }
    pub async fn put_object<'a>(
        &self,
        bucket: &str,
        name: &str,
        content_type: &Mime,
        stream: &'a mut dyn Read,
        size: usize,
    ) -> Result<()> {
        let mut req = PutObjectArgs::new(bucket, name, stream, Some(size), None)?;
        let content_type = content_type.to_string();
        req.content_type = &content_type;
        self.client.put_object(&mut req).await?;
        Ok(())
    }
    pub async fn remove_object(&self, bucket: &str, name: &str) -> Result<()> {
        self.client
            .remove_object(&RemoveObjectArgs::new(bucket, name)?)
            .await?;

        Ok(())
    }
    pub async fn list_objects(&self, bucket: &str) -> Result<Vec<String>> {
        let res = self
            .client
            .list_objects_v2(&ListObjectsV2Args::new(bucket)?)
            .await?;
        let items = res.contents.iter().map(|x| x.name.clone()).collect();

        Ok(items)
    }

    pub async fn get_presigned_object_url(
        &self,
        bucket: &str,
        name: &str,
        ttl: Duration,
    ) -> Result<String> {
        let arg = {
            let mut it = GetPresignedObjectUrlArgs::new(bucket, name, Method::GET)?;
            it.expiry_seconds = Some(ttl.num_seconds() as u32);
            it
        };

        let res = self.client.get_presigned_object_url(&arg).await?;

        Ok(res.url)
    }
    pub fn get_permanent_object_url(&self, bucket: &str, name: &str) -> Result<String> {
        let url = self.base_url.build_url(
            &Method::GET,
            &"".to_string(),
            &Multimap::new(),
            Some(bucket),
            Some(name),
        )?;

        Ok(url.to_string())
    }
}

#[derive(Template)]
#[template(path = "minio/bucket-public-policy.json", escape = "none")]
pub struct BucketPublicPolicy<'a> {
    pub version: &'a str,
    pub name: &'a str,
}
