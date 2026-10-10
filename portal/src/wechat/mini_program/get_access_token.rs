use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Response {
    pub access_token: String,
    pub expires_in: u16,
}
