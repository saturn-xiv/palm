use std::path::Path;

use serde::{Deserialize, Serialize};

use super::super::Result;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn open(&self) -> Result<Client> {
        todo!()
    }
}

pub struct Client {}

impl Client {
    pub async fn upload<P: AsRef<Path>>(&self, file: P, id: &str) -> Result<()> {
        todo!()
    }
}

// impl super::Provider for Client {
//     async fn upload<P: AsRef<Path>>(&self, file: P, id: &str) -> Result<()> {
//         todo!()
//     }
// }
