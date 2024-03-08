use camelia::orm::postgresql::Config as PostgreSql;
use palm::{
    cache::redis::Config as Redis,
    crypto::Key,
    env::{Environment, Http},
    minio::Config as Minio,
    queue::rabbitmq::Config as RabbitMq,
    search::Config as OpenSearch,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rpc {
    pub host: String,
    pub port: u16,
}
impl Default for Rpc {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub env: Environment,
    #[serde(rename = "cookie-key")]
    pub cookie_key: Key,
    pub musa: Rpc,
    pub orchid: Rpc,
    pub http: Http,
    pub postgresql: PostgreSql,
    pub redis: Redis,
    pub rabbitmq: RabbitMq,
    pub opensearch: OpenSearch,
    pub minio: Minio,
}
