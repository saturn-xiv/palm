use chrono::NaiveDateTime;
use juniper::GraphQLObject;
use portal::opensearch::timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "LavenderPhpFpmStatusItem")]
pub struct Item {
    pub host: String,
    pub pool: String,
    pub process_manager: String,
    pub start_time: i32,
    pub start_since: i32,
    pub accepted_conn: i32,
    pub listen_queue: i32,
    pub max_listen_queue: i32,
    pub listen_queue_len: i32,
    pub idle_processes: i32,
    pub active_processes: i32,
    pub total_processes: i32,
    pub max_active_process: i32,
    pub max_children_reached: i32,
    pub slow_requests: i32,
    pub memory_peak: i32,
    pub processes: Vec<Process>,
    #[serde(with = "timestamp")]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "LavenderPhpFpmStatusProcess")]
pub struct Process {
    pub pid: i32,
    pub state: String,
    pub start_time: i32,
    pub start_since: i32,
    pub requests: i32,
    pub request_duration: i32,
    pub request_method: String,
    pub request_uri: String,
    pub content_length: i32,
    pub user: String,
    pub script: String,
    pub last_request_cpu: f64,
    pub last_request_memory: i32,
}
