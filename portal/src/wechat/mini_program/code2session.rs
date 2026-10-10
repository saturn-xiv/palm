use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Response {
    pub session_key: String,
    pub unionid: String,
    pub openid: String,
    pub errcode: usize,
    pub errmsg: String,
}
