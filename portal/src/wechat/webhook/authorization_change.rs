use serde::{Deserialize, Serialize};

// https://developers.weixin.qq.com/doc/oplatform/Website_App/WeChat_Login/authorization_change.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "ToUserName")]
    pub to_string_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "MsgType")]
    pub r#type: String,
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "CreateTime")]
    pub create_time: usize,
    #[serde(rename = "OpenID")]
    pub open_id: String,
    #[serde(rename = "UnionID")]
    pub union_id: String,
    #[serde(rename = "AppID")]
    pub app_id: String,
    #[serde(rename = "RevokeInfo")]
    pub revoke_info: String,
}
