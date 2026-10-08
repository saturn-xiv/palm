use std::fmt::Debug;

use serde::{Deserialize, Serialize};

// https://docs.opensearch.org/latest/api-reference/document-apis/index-document/#example-response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub _index: String,
    pub _id: String,
    pub _version: usize,
    pub result: String,
    pub _shards: Shards,
    pub _seq_no: usize,
    pub _primary_term: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shards {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
}
