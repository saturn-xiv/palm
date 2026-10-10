use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "client-id")]
    pub client_id: String,
    #[serde(rename = "client-secret")]
    pub client_secret: String,
    #[serde(rename = "redirect-uri")]
    pub redirect_uri: String,
}
