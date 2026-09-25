pub mod controllers;
pub mod graphql;
pub mod models;

use std::any::type_name;
use std::collections::BTreeMap;
use std::env::temp_dir;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

pub struct Plugin;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "jobs-dir", default = "jobs_dir")]
    pub jobs_dir: PathBuf,
    #[serde(rename = "working-dir", default = "working_dir")]
    pub working_dir: PathBuf,
    pub bcc: Vec<String>,
    #[serde(rename = "web-hooks")]
    pub web_hooks: BTreeMap<String, WebHook>,
}

fn jobs_dir() -> PathBuf {
    Path::new(&Component::RootDir).join(type_name::<Plugin>())
}

fn working_dir() -> PathBuf {
    temp_dir().join(type_name::<Plugin>())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebHook {
    #[serde(rename = "gogs")]
    Gogs { email: String, secret: String },
}
