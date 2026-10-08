use std::fmt::Debug;

use serde::{Deserialize, Serialize};

// https://docs.opensearch.org/latest/api-reference/search-apis/search/#example-response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item<T> {
    pub took: usize,
    pub timed_out: bool,
    pub _shards: Shards,
    pub hits: Hits<T>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub count: usize,
    pub _shards: Shards,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Total {
    pub value: usize,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hits<T> {
    pub max_score: Option<f32>,
    pub total: Total,
    pub hits: Vec<Hit<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shards {
    pub total: usize,
    pub successful: usize,
    pub skipped: usize,
    pub failed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit<T> {
    pub _index: String,
    pub _id: String,
    pub _score: Option<f32>,
    pub _source: T,
}
