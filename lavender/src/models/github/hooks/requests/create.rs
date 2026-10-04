use serde::{Deserialize, Serialize};

// https://docs.github.com/en/webhooks/webhook-events-and-payloads#create
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub description: Option<String>,
    pub master_branch: String,
    pub pusher_type: String,
    #[serde(rename = "ref")]
    pub r#ref: String,
    pub ref_type: String,
    pub sender: super::User,
    pub repository: super::Repository,
    pub organization: Option<super::Organization>,
}
