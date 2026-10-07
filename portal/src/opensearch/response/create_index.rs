use serde::{Deserialize, Serialize};

// https://docs.opensearch.org/latest/api-reference/index-apis/create-index/#example-response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    acknowledged: bool,
    shards_acknowledged: bool,
    index: String,
}
