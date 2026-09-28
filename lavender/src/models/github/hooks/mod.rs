pub mod requests;

use axum::http::HeaderMap;
use data_encoding::HEXLOWER;
use hyper::StatusCode;
use portal::{HttpError, Result, get_http_header, hmac::sha256::HmacSha256};
use serde::{Deserialize, Serialize};

// https://docs.github.com/en/webhooks/using-webhooks/creating-webhooks#creating-a-repository-webhook
// https://docs.github.com/en/webhooks/using-webhooks/validating-webhook-deliveries#validating-webhook-deliveries
// https://docs.github.com/en/webhooks/webhook-events-and-payloads#delivery-headers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Header {
    pub delivery: String,
    pub event: String,
    pub signature: String,
    pub signature_256: String,
    pub id: String,
    pub installation_target_type: String,
    pub installation_target_id: String,
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
            signature_256: get_http_header(headers, Self::SIGNATURE_256)
                .unwrap_or_default()
                .to_string(),
            id: get_http_header(headers, Self::ID)
                .unwrap_or_default()
                .to_string(),
            installation_target_id: get_http_header(headers, Self::INSTALLATION_TARGET_ID)
                .unwrap_or_default()
                .to_string(),
            installation_target_type: get_http_header(headers, Self::INSTALLATION_TARGET_TYPE)
                .unwrap_or_default()
                .to_string(),
        }
    }
    pub fn verify(&self, secret: &str, body: &str) -> Result<()> {
        if let Some(signature) = self.signature_256.strip_prefix("sha256=") {
            let it = {
                let md = HmacSha256::new(secret.as_bytes())?;
                let buf = md.sign(body.as_bytes())?;
                HEXLOWER.encode(&buf)
            };
            if signature == it {
                return Ok(());
            }
        }
        Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
    }
    const SIGNATURE_256: &str = "X-Hub-Signature-256";
    const SIGNATURE: &str = "X-Hub-Signature";
    const ID: &str = "X-GitHub-Hook-ID";
    const EVENT: &str = "X-GitHub-Event";
    const DELIVERY: &str = "X-GitHub-Delivery";
    const INSTALLATION_TARGET_TYPE: &str = "X-GitHub-Hook-Installation-Target-Type";
    const INSTALLATION_TARGET_ID: &str = "X-GitHub-Hook-Installation-Target-ID";
}
