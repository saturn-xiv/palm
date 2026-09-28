use serde::{Deserialize, Serialize};

// https://docs.github.com/en/webhooks/webhook-events-and-payloads#push
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub after: String,
    pub base_ref: Option<String>,
    pub before: String,
    pub commits: Vec<Commit>,
    pub compare: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub head_commit: Option<Commit>,
    pub organization: super::Organization,
    pub pusher: Pusher,
    #[serde(rename = "ref")]
    pub r#ref: String,
    pub repository: super::Repository,
    pub sender: super::User,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pusher {
    pub date: Option<String>,
    pub email: Option<String>,
    pub name: String,
    pub username: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commit {
    pub added: Option<Vec<String>>,
    pub author: User,
    pub commiter: User,
    pub distinct: bool,
    pub id: String,
    pub message: String,
    pub modified: Option<Vec<String>>,
    pub removed: Option<Vec<String>>,
    pub timestamp: String,
    pub tree_id: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub date: Option<String>,
    pub email: Option<String>,
    pub name: String,
    pub username: Option<String>,
}
