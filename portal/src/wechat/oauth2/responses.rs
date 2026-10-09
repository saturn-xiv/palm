use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub openid: String,
    pub scope: String,
    pub unionid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub access_token: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub openid: String,
    pub scope: String,
}

// https://developers.weixin.qq.com/community/develop/doc/00028edbe3c58081e7cc834705b801?blockType=1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub openid: String,
    pub nickname: String,
    pub headimgurl: Option<String>,
    pub privilege: Vec<String>,
    pub unionid: String,
}
