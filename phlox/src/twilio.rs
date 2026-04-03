use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};
use twilio::apis::{
    Error as TwilioError,
    api20100401_message_api::{CreateMessageError, CreateMessageParams, create_message},
    configuration::Configuration,
};

// https://www.twilio.com/docs/openapi/generating-a-rust-client-for-twilios-api#send-an-sms-using-twilio-and-rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "account-sid")]
    pub account_sid: String,
    #[serde(rename = "api-key")]
    pub api_key: String,
    #[serde(rename = "api-key-secret")]
    pub api_key_secret: String,
    #[serde(rename = "phone-number")]
    pub phone_number: String,
}

impl Node {
    pub async fn sms(
        &self,
        to: String,
        body: String,
        status_callback: Option<String>,
    ) -> StdResult<(), TwilioError<CreateMessageError>> {
        log::info!("send sms to {}", to);
        let config = Configuration {
            basic_auth: Some((self.api_key.clone(), Some(self.api_key_secret.clone()))),
            ..Default::default()
        };

        let message = CreateMessageParams {
            account_sid: self.account_sid.clone(),
            to,
            from: Some(self.phone_number.clone()),
            body: Some(body),
            status_callback,
            application_sid: None,
            max_price: None,
            provide_feedback: None,
            attempt: None,
            validity_period: None,
            force_delivery: None,
            content_retention: None,
            content_sid: None,
            content_variables: None,
            address_retention: None,
            smart_encoded: None,
            persistent_action: None,
            send_as_mms: None,
            send_at: None,
            shorten_urls: None,
            schedule_type: None,
            traffic_type: None,
            risk_check: None,
            messaging_service_sid: None,
            media_url: None,
        };

        let message = create_message(&config, message).await?;
        log::debug!("response: {:?}", message);
        Ok(())
    }
}

impl From<Node> for Configuration {
    fn from(item: Node) -> Self {
        Self {
            basic_auth: Some((item.api_key, Some(item.api_key_secret))),
            ..Default::default()
        }
    }
}
