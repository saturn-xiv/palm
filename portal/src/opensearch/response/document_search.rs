use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    took: usize,
    timed_out: bool,
    hits: Hits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Total {
    value: usize,
    relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hits {
    max_score: f32,
    total: Total,
    hits: Vec<Value>,
}
