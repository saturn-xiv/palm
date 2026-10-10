use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Response {
    pub errcode: usize,
    pub errmsg: String,
    pub openid: String,
    pub session_key: String,
}
