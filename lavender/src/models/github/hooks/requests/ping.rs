use serde::{Deserialize, Deserializer, Serialize, de::Error as DeError};
use serde_json::Value;

// https://docs.github.com/en/webhooks/webhook-events-and-payloads#ping
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub hook: Hook,
    pub hook_id: u32,
    pub organization: super::Organization,
    pub repository: super::Repository,
    pub sender: super::User,
    pub zen: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hook {
    pub active: bool,
    pub app_id: Option<u32>,
    pub config: Config,
    pub created_at: String,
    pub deliveries_url: Option<String>,
    pub events: Vec<String>,
    pub id: u32,
    pub last_response: Option<LastResponse>,
    pub name: String,
    pub ping_url: Option<String>,
    pub test_url: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub updated_at: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub content_type: Option<String>,
    pub secret: Option<String>,
    #[serde(deserialize_with = "string_or_number")]
    pub insecure_ssl: u8,
    pub url: Option<String>,
}

fn string_or_number<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
    D::Error: DeError,
{
    match Value::deserialize(deserializer)? {
        Value::Number(n) => {
            let it = n
                .as_u64()
                .ok_or_else(|| DeError::custom("invalid number format"))?;
            Ok(it as u8)
        }
        Value::String(s) => s.parse::<u8>().map_err(serde::de::Error::custom),
        _ => Err(DeError::custom("expected a string or a number")),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastResponse {
    pub code: Option<u32>,
    pub status: Option<String>,
    pub message: Option<String>,
}
