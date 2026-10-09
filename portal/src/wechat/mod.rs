pub mod mini_program;
pub mod oauth2;
pub mod webhook;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "errcode")]
    pub code: usize,
    #[serde(rename = "errmsg")]
    pub message: String,
}
