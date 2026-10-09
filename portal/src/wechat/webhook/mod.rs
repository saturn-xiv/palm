pub mod authorization_change;
pub mod ping;

use data_encoding::{BASE64, HEXLOWER};
use hyper::StatusCode;
use openssl::sha::sha1;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::super::{HttpError, Result};

// https://developers.weixin.qq.com/doc/oplatform/Website_App/WeChat_Login/message_push.html
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "encoding-aes-key")]
    pub encoding_aes_key: String,
}

impl Config {
    pub fn decrypt<T: DeserializeOwned>(&self, message: &str) -> Result<T> {
        todo!()
    }

    fn key(&self) -> Result<Vec<u8>> {
        let it = {
            let k = format!("{}=", self.encoding_aes_key);
            BASE64.decode(k.as_bytes())?
        };
        Ok(it)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "ToUserName")]
    pub to_string_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "Encrypt")]
    pub encrypt: String,
    #[serde(rename = "MsgSignature")]
    pub signature: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: i64,
    #[serde(rename = "Nonce")]
    pub nonce: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Query {
    pub signature: String,
    pub timestamp: String,
    pub nonce: String,
    pub encrypt_type: String,
    pub msg_signature: String,
    pub openid: String,
}

impl Query {
    pub fn verify(&self, token: &str, encrypt: &str) -> Result<()> {
        let data = {
            let mut items = [self.timestamp.as_str(), self.nonce.as_str(), encrypt, token];
            items.sort();
            items.join("")
        };
        let buf = sha1(data.as_bytes());
        if self.msg_signature != HEXLOWER.encode(&buf) {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("invalid signature".to_string()),
            )));
        }
        Ok(())
    }
}
