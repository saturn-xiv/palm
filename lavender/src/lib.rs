pub mod graphql;
pub mod models;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "jobs-dir", default = "jobs_dir")]
    pub jobs_dir: String,
    #[serde(rename = "working-dir", default = "working_dir")]
    pub working_dir: String,
    pub bcc: Vec<String>,
}

fn jobs_dir() -> String {
    "/etc/lavender/jobs".to_string()
}

fn working_dir() -> String {
    "/var/tmp/lavender".to_string()
}
