use petunia::{crypto::Key, opensearch::Config as OpenSearch, Result};
use serde::{Deserialize, Serialize};

#[derive(clap::Parser, PartialEq, Eq, Debug)]
pub struct Command {
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub secrets: Key,
    #[serde(rename = "systemd-journal-index-name")]
    pub systemd_journal_index_name: String,
    #[serde(rename = "snmp-index-name")]
    pub snmp_index_name: String,
    #[serde(rename = "open-search")]
    pub open_search: OpenSearch,
}

pub fn launch(config: &Config, port: u16) -> Result<()> {
    Ok(())
}
