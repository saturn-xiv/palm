use serde::{Deserialize, Serialize};

// https://docs.opensearch.org/latest/api-reference/search-apis/count/#example-response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub count: usize,
    pub _shards: Shards,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shards {
    pub total: usize,
    pub successful: usize,
    pub skipped: usize,
    pub failed: usize,
}
