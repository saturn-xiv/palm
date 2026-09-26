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
pub struct Config {
    #[serde(rename = "access-key-id")]
    pub access_key_id: String,
    #[serde(rename = "secret-access-key")]
    pub secret_access_key: String,
    pub region: String,
    pub namespace: Option<String>,
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
            namespace: self.namespace.clone(),
        })
    }
}

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
pub struct Client {
    s3: S3Client,
    region: String,
    namespace: Option<String>,
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

    pub fn bucket(&self, name: &str) -> String {
        match self.namespace {
            Some(ref it) => format!("{it}.{name}"),
            None => name.to_string(),
        }
    }

    fn ttl_tag() -> Result<Tag> {
        let it = Tag::builder().key("ttl").value("yes").build()?;
        Ok(it)
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
}
