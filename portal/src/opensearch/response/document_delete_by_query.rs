use serde::{Deserialize, Serialize};

// https://docs.opensearch.org/latest/api-reference/document-apis/delete-by-query/#example-response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub took: usize,
    pub timed_out: bool,
    pub total: usize,
    pub deleted: usize,
    pub batches: usize,
    pub version_conflicts: usize,
    pub noops: usize,
    pub throttled_millis: usize,
    pub requests_per_second: f32,
    pub throttled_until_millis: usize,
}
