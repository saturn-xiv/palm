pub mod responses;

use hyper::StatusCode;
use icu::locale::Locale;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::Response;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::super::{HttpError, Result};
use super::ErrorResponse;

// https://developers.weixin.qq.com/doc/oplatform/Website_App/WeChat_Login/Wechat_Login.html
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "app-id")]
    pub app_id: String,
    #[serde(rename = "app-secret")]
    pub app_secret: String,
}

impl Config {
    pub fn login_url(&self, redirect_uri: &str, state: &str, lang: &Locale) -> String {
        format!(
            "https://open.weixin.qq.com/connect/qrconnect?appid={}&redirect_uri={}&response_type=code&scope=snsapi_login&state={}&lang={}#wechat_redirect",
            self.app_id,
            utf8_percent_encode(redirect_uri, NON_ALPHANUMERIC),
            state,
            if lang.id.language.as_str() == "zh" {
                "cn"
            } else {
                "en"
            }
        )
    }

    pub async fn access_token(&self, code: &str) -> Result<responses::AccessToken> {
        let url = format!(
            "https://api.weixin.qq.com/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code",
            self.app_id, self.app_secret, code
        );
        let res = reqwest::get(url).await?;
        Self::json(res).await
    }

    pub async fn is_access_token_valid(&self, openid: &str, access_token: &str) -> Result<bool> {
        let url = format!(
            "https://api.weixin.qq.com/sns/auth?access_token={access_token}&openid={openid}"
        );
        let res = reqwest::get(url).await?;
        let it: ErrorResponse = res.json().await?;
        log::debug!("{:?}", it);
        Ok(it.code == 0)
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<responses::RefreshToken> {
        let url = format!(
            "https://api.weixin.qq.com/sns/oauth2/refresh_token?appid={}&grant_type=refresh_token&refresh_token={}",
            self.app_id, refresh_token
        );
        let res = reqwest::get(url).await?;
        Self::json(res).await
    }

    pub async fn user_info(
        &self,
        open_id: &str,
        access_token: &str,
    ) -> Result<responses::UserInfo> {
        let url = format!(
            "https://api.weixin.qq.com/sns/userinfo?access_token={access_token}&openid={open_id}"
        );
        let res = reqwest::get(url).await?;
        Self::json(res).await
    }

    async fn json<T: DeserializeOwned>(res: Response) -> Result<T> {
        let status = res.status();
        if !status.is_success() {
            let it: ErrorResponse = res.json().await?;
            log::error!("{:?}", it);
            return Err(Box::new(HttpError(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(it.message),
            )));
        }
        Ok(res.json().await?)
    }
}
