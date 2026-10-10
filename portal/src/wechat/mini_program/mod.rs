pub mod code2session;
pub mod get_access_token;
pub mod get_stable_access_token;
pub mod reset_user_session_key;

use data_encoding::HEXLOWER;
use serde::{Deserialize, Serialize};

use super::super::{Result, hmac::sha256::HmacSha256};

// https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/login.html
// https://developers.weixin.qq.com/miniprogram/dev/server/API/user-login/
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "app-id")]
    pub app_id: String,
    #[serde(rename = "app-secret")]
    pub app_secret: String,
}

impl Config {
    pub async fn get_stable_access_token(
        &self,
        force_refresh: bool,
    ) -> Result<get_stable_access_token::Response> {
        let cli = reqwest::Client::new();
        let res = cli
            .post("https://api.weixin.qq.com/cgi-bin/stable_token")
            .json(&get_stable_access_token::Request::new(
                &self.app_id,
                &self.app_secret,
                force_refresh,
            ))
            .send()
            .await?;
        let it: get_stable_access_token::Response = res.json().await?;
        Ok(it)
    }
    pub async fn get_access_token(&self) -> Result<get_access_token::Response> {
        let res = reqwest::get(format!(
            "https://api.weixin.qq.com/cgi-bin/token?appid={}&secret={}&grant_type=client_credential",
            self.app_id, self.app_secret,
        )).await?;
        let it: get_access_token::Response = res.json().await?;
        Ok(it)
    }
    pub async fn code2session(&self, code: &str) -> Result<code2session::Response> {
        let res = reqwest::get(format!(
            "https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
            self.app_id, self.app_secret, code
        )).await?;
        let it: code2session::Response = res.json().await?;
        Ok(it)
    }
    pub async fn check_session_key(
        &self,
        session_key: &str,
        access_token: &str,
    ) -> Result<super::ErrorResponse> {
        let cli = reqwest::Client::new();
        let res = cli
            .get(format!(
                "https://api.weixin.qq.com/wxa/checksession?access_token={access_token}",
            ))
            .json(&SessionKeySignatureRequest::new(&self.app_id, session_key)?)
            .send()
            .await?;
        let it: super::ErrorResponse = res.json().await?;
        Ok(it)
    }
    pub async fn reset_user_session_key(
        &self,
        session_key: &str,
        access_token: &str,
    ) -> Result<reset_user_session_key::Response> {
        let cli = reqwest::Client::new();
        let res = cli
            .get(format!(
                "https://api.weixin.qq.com/wxa/resetusersessionkey?access_token={access_token}",
            ))
            .json(&SessionKeySignatureRequest::new(&self.app_id, session_key)?)
            .send()
            .await?;
        let it = res.json().await?;
        Ok(it)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionKeySignatureRequest {
    pub openid: String,
    pub signature: String,
    pub sig_method: String,
}

impl SessionKeySignatureRequest {
    pub fn new(open_id: &str, session_key: &str) -> Result<Self> {
        let it = Self {
            openid: open_id.to_string(),
            signature: {
                let it = HmacSha256::new(session_key.as_bytes())?;
                let buf = it.sign("".as_bytes())?;
                HEXLOWER.encode(&buf)
            },
            sig_method: "hmac_sha256".to_string(),
        };
        Ok(it)
    }
}
