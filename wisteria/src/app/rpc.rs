use std::path::Path;

use portal::{
    Result, cache::redis::Node as Redis, is_stopped, minio::Node as Minio,
    open_search::Node as OpenSearch, orm::postgresql::Node as PostgreSql, parse_toml,
    queue::rabbitmq::Node as RabbitMq,
};
use serde::{Deserialize, Serialize};

pub async fn start<P: AsRef<Path>>(config: P, _port: u16) -> Result<()> {
    if is_stopped()? {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let _config: Config = parse_toml(config)?;
    // TODO
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    postgresql: PostgreSql,
    rabbitmq: RabbitMq,
    redis: Redis,
    minio: Minio,
    opensearch: OpenSearch,
}
