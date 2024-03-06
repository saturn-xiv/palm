use serde::{Deserialize, Serialize};
use twilio::{Client, OutboundMessage};

use super::Result;

pub mod v1 {
    tonic::include_proto!("palm.sms.v1");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub account_sid: String,
    pub auth_token: String,
    pub from: String,
}

impl Config {
    pub async fn send_sms(&self, to: &str, message: &str) -> Result<()> {
        let client = self.open();
        let message = client
            .send_message(OutboundMessage::new(&self.from, to, message))
            .await?;
        info!("{:?}", message);
        Ok(())
    }
    fn open(&self) -> Client {
        Client::new(&self.account_sid, &self.auth_token)
    }
}
