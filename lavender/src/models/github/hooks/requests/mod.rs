pub mod ping;
pub mod push;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Organization {
    pub id: u32,
    pub login: String,
    pub node_id: String,
    pub avatar_url: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub login: String,
    pub node_id: String,
    pub avatar_url: String,
    #[serde(rename = "type")]
    pub r#type: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub id: u32,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub default_branch: String,
}
