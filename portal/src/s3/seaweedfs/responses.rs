use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssignResponse {
    pub fid: String,
    pub url: String,
    pub public_url: String,
    pub count: u8,
}
