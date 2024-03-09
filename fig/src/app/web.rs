use std::path::Path;

use camelia::orm::postgresql::Config as PostgreSql;
use clap::Parser;
use palm::{
    cache::redis::Config as Redis,
    crypto::{
        tink::{Aes, HMac},
        Key,
    },
    env::{Environment, Http},
    minio::Config as Minio,
    queue::rabbitmq::Config as RabbitMq,
    search::Config as OpenSearch,
    Result,
};
use serde::{Deserialize, Serialize};

use super::super::NAME;

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Server {
    #[clap(short, long)]
    pub port: u16,
    #[clap(short, long)]
    pub threads: usize,
}

impl Server {
    pub async fn launch<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}

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
    #[serde(rename = "secret-key")]
    pub secret_key: Key,
    pub musa: Rpc,
    pub orchid: Rpc,
    pub http: Http,
    pub postgresql: PostgreSql,
    pub redis: Redis,
    pub rabbitmq: RabbitMq,
    pub opensearch: OpenSearch,
    pub minio: Minio,
}

impl Config {
    pub fn hmac() -> Result<HMac> {
        HMac::new(format!("{}-hmac.bin", NAME))
    }
    pub fn aes() -> Result<Aes> {
        Aes::new(format!("{}-aes.bin", NAME))
    }
}
