use std::default::Default;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Environment {
    Production,
    Staging,
    #[default]
    Development,
    Test,
}

impl fmt::Display for Environment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Production => fmt.write_str("production"),
            Environment::Staging => fmt.write_str("staging"),
            Environment::Development => fmt.write_str("development"),
            Environment::Test => fmt.write_str("test"),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    #[serde(rename = "allow-origins")]
    pub allow_origins: Vec<String>,
}

impl Http {
    pub fn addr(port: u16) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rpc;

impl Rpc {
    pub fn addr(port: u16) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)
    }
}
