use std::ops::DerefMut;
use std::path::Path;

use palm::{
    cache::{redis::Config as Redis, Provider as CacheProvider},
    parser::from_toml,
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub redis: Redis,
}

pub fn list<P: AsRef<Path>>(config_file: P) -> Result<()> {
    let config: Config = from_toml(config_file)?;
    let ch = config.redis.open()?;
    {
        let mut ch = ch.get()?;
        let ch = ch.deref_mut();
        let items = ch.keys()?;
        println!("{:<14} KEY", "TTL");
        for (node, key, ttl) in items.iter() {
            println!("{:<32} {:<14} {}", node, ttl, key);
        }
    }
    Ok(())
}

pub fn clear<P: AsRef<Path>>(config_file: P) -> Result<()> {
    let config: Config = from_toml(config_file)?;
    let ch = config.redis.open()?;
    {
        let mut ch = ch.get()?;
        let ch = ch.deref_mut();
        ch.clear()?;
    }
    Ok(())
}
