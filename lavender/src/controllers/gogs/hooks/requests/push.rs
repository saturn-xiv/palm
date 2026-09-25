use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    #[serde(rename = "ref")]
    pub r#ref: String,
    pub before: String,
    pub after: String,
    pub compare_url: String,
    pub commits: Vec<Commit>,
    pub repository: Repository,
    pub pusher: Pusher,
    pub sender: Sender,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub url: String,
    pub author: CommitAuthor,
    pub committer: CommitCommitter,
    pub added: Option<Vec<String>>,
    pub removed: Option<Vec<String>>,
    pub modified: Option<Vec<String>>,
    pub timestamp: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub username: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitCommitter {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub id: u16,
    pub owner: RepositoryOwner,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub private: bool,
    pub fork: bool,
    pub parent: Option<String>,
    pub empty: bool,
    pub mirror: bool,
    pub size: usize,
    pub html_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub website: String,
    pub stars_count: u16,
    pub forks_count: u16,
    pub watchers_count: u16,
    pub open_issues_count: u16,
    pub default_branch: String,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryOwner {
    pub id: u16,
    pub login: String,
    pub full_name: String,
    pub email: String,
    pub avatar_url: String,
    pub username: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pusher {
    pub id: u16,
    pub login: String,
    pub full_name: String,
    pub email: String,
    pub avatar_url: String,
    pub username: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sender {
    pub id: u16,
    pub login: String,
    pub full_name: String,
    pub email: String,
    pub avatar_url: String,
    pub username: String,
}
