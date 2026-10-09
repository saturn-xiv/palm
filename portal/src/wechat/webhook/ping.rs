use data_encoding::HEXLOWER;
use hyper::StatusCode;
use openssl::sha::sha1;
use serde::Deserialize;

use super::super::super::{HttpError, Result};

#[derive(Debug, Clone, Deserialize)]
pub struct Query {
    pub signature: String,
    pub timestamp: String,
    pub nonce: String,
    #[serde(rename = "echostr")]
    pub echo_str: String,
}

impl Query {
    pub fn verify(&self, token: &str) -> Result<()> {
        let data = {
            let mut items = [self.timestamp.as_str(), self.nonce.as_str(), token];
            items.sort();
            items.join("")
        };
        let buf = sha1(data.as_bytes());
        if self.signature != HEXLOWER.encode(&buf) {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("invalid signature".to_string()),
            )));
        }
        Ok(())
    }
}
