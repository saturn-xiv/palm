use std::ops::DerefMut;
use std::path::Path;

use camelia::orm::postgresql::Config as PostgreSql;
use palm::{
    cache::redis::Config as Redis, parser::from_toml, queue::rabbitmq::Config as RabbitMq,
    search::Config as OpenSearch, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub postgresql: PostgreSql,
    pub opensearch: OpenSearch,
    pub redis: Redis,
    pub rabbitmq: RabbitMq,
}

pub fn seed<P: AsRef<Path>>(config_file: P) -> Result<()> {
    let config: Config = from_toml(config_file)?;
    let db = config.postgresql.open()?;
    // TODO
    {
        let mut db = db.get()?;
        let _db = db.deref_mut();
    }
    Ok(())
}
