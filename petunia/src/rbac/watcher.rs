use casbin::{CoreApi, Enforcer};
use prost::Message;
use tokio::sync::Mutex;

use super::super::{queue::amqp::Handler as RabbitMqConsumerHandler, Result};
use super::v1::WatcherMessage;

impl RabbitMqConsumerHandler for Mutex<Enforcer> {
    async fn handle(&self, _id: &str, _content_type: &str, payload: &[u8]) -> Result<()> {
        WatcherMessage::decode(payload)?;
        let mut it = self.lock().await;
        it.load_policy().await?;
        Ok(())
    }
}
