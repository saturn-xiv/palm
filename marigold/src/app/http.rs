use std::path::Path;

use clap::ValueEnum;
use phlox::{Result, cache::redis::Node as Redis, is_stopped, minio::Node as Minio, parse_toml};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumDisplay,
    EnumString,
    ValueEnum,
    Default,
)]
pub enum Theme {
    #[strum(serialize = "bulma")]
    Bulma,
    #[default]
    #[strum(serialize = "bootstrap")]
    Bootstrap,
}
// impl fmt::Display for Theme {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

pub async fn start<P: AsRef<Path>>(config: P, _port: u16, _theme: Theme) -> Result<()> {
    if is_stopped() {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let _config: Config = parse_toml(config)?;
    // TODO
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    redis: Redis,
    minio: Minio,
    phlox: Rpc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rpc {
    pub host: String,
    pub port: u16,
}
