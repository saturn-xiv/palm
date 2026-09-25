pub mod requests;

use axum::http::HeaderMap;
use data_encoding::HEXLOWER;
use hyper::StatusCode;
use portal::{HttpError, Result, get_http_header, hmac::sha256::HmacSha256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Header {
    pub delivery: String,
    pub event: String,
    pub signature: String,
}

impl Header {
    pub fn new(headers: &HeaderMap) -> Self {
        Self {
            delivery: get_http_header(headers, Self::DELIVERY)
                .unwrap_or_default()
                .to_string(),
            event: get_http_header(headers, Self::EVENT)
                .unwrap_or_default()
                .to_string(),
            signature: get_http_header(headers, Self::SIGNATURE)
                .unwrap_or_default()
                .to_string(),
        }
    }

    pub fn verify(&self, secret: &str, body: &str) -> Result<()> {
        let it = {
            let md = HmacSha256::new(secret.as_bytes())?;
            let buf = md.sign(body.as_bytes())?;
            HEXLOWER.encode(&buf)
        };
        if self.signature != it {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
        Ok(())
    }

    const DELIVERY: &str = "X-Gogs-Delivery";
    const EVENT: &str = "X-Gogs-Event";
    const SIGNATURE: &str = "X-Gogs-Signature";
}
