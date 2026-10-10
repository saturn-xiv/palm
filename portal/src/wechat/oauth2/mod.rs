pub mod responses;

use icu::locale::Locale;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

use serde::{Deserialize, Serialize};

use super::super::Result;
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
        let res = reqwest::get(format!(
            "https://api.weixin.qq.com/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code",
            self.app_id, self.app_secret, code
        )).await?;
        super::to_json(res).await
    }

    pub async fn is_access_token_valid(&self, openid: &str, access_token: &str) -> Result<bool> {
        let res = reqwest::get(format!(
            "https://api.weixin.qq.com/sns/auth?access_token={access_token}&openid={openid}"
        ))
        .await?;
        let it: ErrorResponse = res.json().await?;
        log::debug!("{:?}", it);
        Ok(it.code == 0)
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<responses::RefreshToken> {
        let res = reqwest::get(format!(
            "https://api.weixin.qq.com/sns/oauth2/refresh_token?appid={}&grant_type=refresh_token&refresh_token={}",
            self.app_id, refresh_token
        )).await?;
        super::to_json(res).await
    }

    pub async fn user_info(
        &self,
        open_id: &str,
        access_token: &str,
    ) -> Result<responses::UserInfo> {
        let res = reqwest::get(format!(
            "https://api.weixin.qq.com/sns/userinfo?access_token={access_token}&openid={open_id}"
        ))
        .await?;
        super::to_json(res).await
    }
}
