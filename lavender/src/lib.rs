pub mod controllers;
pub mod graphql;
pub mod models;

use std::any::type_name;
use std::collections::BTreeMap;
use std::env::temp_dir;
use std::path::{Component, Path, PathBuf};

use hyacinth::lavender_v1;
use portal::{
    Result,
    opensearch::Client as OpenSearch,
    queue::rabbitmq::{Client as RabbitMq, QueueDeclareOptions},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct Plugin;

impl Plugin {
    pub async fn init(queue: &RabbitMq, search: &OpenSearch) -> Result<()> {
        queue
            .declare_queue(
                type_name::<models::job::Message>(),
                QueueDeclareOptions {
                    durable: true,
                    exclusive: true,
                    ..Default::default()
                },
            )
            .await?;
        if !search.index_exists::<lavender_v1::SystemdRequest>().await? {
            search
                .create_index::<lavender_v1::SystemdRequest>(
                    None,
                    Some(json!({
                        "properties":{
                            "host": {"type": "keyword"},
                            "unit": {"type": "keyword"},
                            "priority": {"type": "byte"},
                            "message": {"type": "text"},
                            "created_at": {"type": "date_nanos", "format": "strict_date_optional_time_nanos"}
                        }
                    })),
                )
                .await?;
        }
        if !search
            .index_exists::<lavender_v1::KubernetesRequest>()
            .await?
        {
            search
                .create_index::<lavender_v1::KubernetesRequest>(
                    None,
                    Some(json!({
                        "properties":{
                            "node": {"type": "keyword"},
                            "pod": {"type": "keyword"},
                            "container": {"type": "keyword"},
                            "message": {"type": "text"},
                            "created_at": {"type": "date_nanos", "format": "strict_date_optional_time_nanos"}
                        }
                    })),
                )
                .await?;
        }
        if !search
            .index_exists::<lavender_v1::http_request::Item>()
            .await?
        {
            search
                .create_index::<lavender_v1::http_request::Item>(
                    None,
                    Some(json!({
                        "properties":{
                            "url": {"type": "keyword"},
                            "from": {"type": "keyword"},
                            "status_code": {"type": "short"},
                            "content_type": {"type": "keyword"},
                            "body": {"type": "text"},
                            "elapsed": {"type": "date_nanos", "format": "strict_date_optional_time_nanos"},
                            "created_tt": {"type": "date_nanos", "format": "strict_date_optional_time_nanos"}
                        }
                    })),
                )
                .await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "jobs-dir", default = "jobs_dir")]
    pub jobs_dir: PathBuf,
    #[serde(rename = "working-dir", default = "working_dir")]
    pub working_dir: PathBuf,
    pub bcc: Vec<String>,
    #[serde(rename = "web-hooks")]
    pub web_hooks: BTreeMap<String, WebHook>,
}

fn jobs_dir() -> PathBuf {
    Path::new(&Component::RootDir).join(type_name::<Plugin>())
}

fn working_dir() -> PathBuf {
    temp_dir().join(type_name::<Plugin>())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebHook {
    #[serde(rename = "gogs")]
    Gogs { email: String, secret: String },
    #[serde(rename = "github")]
    Github { email: String, secret: String },
}
