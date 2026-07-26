use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(default = "node_default_host")]
    pub host: String,
    #[serde(default = "node_default_port")]
    pub port: u16,
    #[serde(default = "node_default_user")]
    pub user: String,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(rename = "db-name")]
    pub db_name: String,
    #[serde(rename = "pool-size", default = "node_default_pool_size")]
    pub pool_size: usize,
}

fn node_default_host() -> String {
    "127.0.0.1".to_string()
}

fn node_default_user() -> String {
    "root".to_string()
}

fn node_default_port() -> u16 {
    3306
}

fn node_default_pool_size() -> usize {
    32
}
