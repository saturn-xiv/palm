use serde::{Deserialize, Serialize};

pub use super::get_access_token::Response;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Request {
    pub secret: String,
    pub appid: String,
    pub grant_type: String,
    pub force_refresh: bool,
}

impl Request {
    pub fn new(app_id: &str, app_secret: &str, force_refresh: bool) -> Self {
        Self {
            secret: app_secret.to_string(),
            appid: app_id.to_string(),
            grant_type: "client_credential".to_string(),
            force_refresh,
        }
    }
}
