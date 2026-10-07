use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub took: usize,
    pub timed_out: bool,
    pub hits: Hits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Total {
    pub value: usize,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hits {
    pub max_score: f32,
    pub total: Total,
    pub hits: Vec<Value>,
}
