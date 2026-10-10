use serde::{Deserialize, Serialize};

// https://developers.google.com/identity/protocols/oauth2/web-server#httprest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "client-id")]
    pub client_id: String,
    #[serde(rename = "client-secret")]
    pub client_secret: String,
    #[serde(rename = "redirect-url")]
    pub redirect_url: String,
}
