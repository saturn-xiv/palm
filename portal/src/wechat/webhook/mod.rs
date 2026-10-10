pub mod authorization_change;
pub mod ping;

use chrono::Utc;
use data_encoding::{BASE64, HEXLOWER};
use hyper::StatusCode;
use openssl::{
    sha::sha1,
    symm::{Cipher, decrypt, encrypt},
};
use rand::RngExt;
use serde::{Deserialize, Serialize};

use super::super::{HttpError, Result};

// https://developers.weixin.qq.com/doc/oplatform/Website_App/WeChat_Login/message_push.html
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "encoding-aes-key")]
    pub encoding_aes_key: String,
}

impl Config {
    pub fn response<T: Serialize>(&self, app_id: &str, message: &T) -> Result<Response> {
        let nonce = {
            let mut rng = rand::rng();
            let x: u32 = rng.random_range(1_000_000_000..u32::MAX);
            x
        };
        let msg_encrypt = {
            let key = self.key()?;
            let data = {
                let msg = serde_json::to_vec(message)?;
                let msg_len = {
                    let i = msg.len() as i32;
                    let it: [u8; 4] = i.to_be_bytes();
                    it
                };
                let salt = {
                    let mut it = vec![0u8; Self::SALT_LEN];
                    rand::rng().fill(&mut it[..]);
                    it
                };
                let mut buf = Vec::new();

                buf.extend(salt);
                buf.extend(&msg_len);
                buf.extend(&msg);
                buf.extend(app_id.as_bytes());
                buf
            };

            let cipher = Cipher::aes_128_cbc();
            let buf = encrypt(cipher, &key, None, &data)?;
            BASE64.encode(&buf)
        };
        let timestamp = Utc::now().timestamp();
        let signature = {
            let timestamp = timestamp.to_string();
            let nonce = nonce.to_string();
            let data = {
                let mut items = [
                    timestamp.as_str(),
                    nonce.as_str(),
                    msg_encrypt.as_str(),
                    self.token.as_str(),
                ];
                items.sort();
                items.join("")
            };
            let buf = sha1(data.as_bytes());
            HEXLOWER.encode(&buf)
        };

        Ok(Response {
            encrypt: msg_encrypt,
            timestamp,
            nonce,
            signature,
        })
    }
    pub fn request(&self, app_id: &str, message: &str) -> Result<Request> {
        let key = self.key()?;
        let message = BASE64.decode(message.as_bytes())?;
        let cipher = Cipher::aes_128_cbc();
        let buf = decrypt(cipher, &key, None, &message)?;
        if buf.len() <= Self::SALT_LEN + Self::MESSAGE_LEN {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("message length is too short".to_string()),
            )));
        }
        let msg_len = {
            let tmp = &buf[Self::SALT_LEN..Self::SALT_LEN + Self::MESSAGE_LEN];
            let it: [u8; 4] = tmp.try_into()?;
            i32::from_be_bytes(it) as usize
        };
        if buf.len() != Self::SALT_LEN + Self::MESSAGE_LEN + msg_len + app_id.len() {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("invalid message length {}", buf.len())),
            )));
        }
        if std::str::from_utf8(&buf[Self::SALT_LEN + Self::MESSAGE_LEN + msg_len..])? != app_id {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("invalid Appid part".to_string()),
            )));
        }
        let it = serde_json::from_slice(
            &buf[Self::SALT_LEN + Self::MESSAGE_LEN..Self::SALT_LEN + Self::MESSAGE_LEN + msg_len],
        )?;
        Ok(it)
    }

    const SALT_LEN: usize = 16;
    const MESSAGE_LEN: usize = 4;
    fn key(&self) -> Result<Vec<u8>> {
        let it = {
            let k = format!("{}=", self.encoding_aes_key);
            BASE64.decode(k.as_bytes())?
        };
        Ok(it)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EncryptRequest {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "Encrypt")]
    pub encrypt: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Request {
    AuthorizationChange(authorization_change::Item),
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
    pub nonce: u32,
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
        let signature = {
            let data = {
                let mut items = [self.timestamp.as_str(), self.nonce.as_str(), encrypt, token];
                items.sort();
                items.join("")
            };
            let buf = sha1(data.as_bytes());
            HEXLOWER.encode(&buf)
        };
        if self.msg_signature != signature {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("invalid signature".to_string()),
            )));
        }
        Ok(())
    }
}
