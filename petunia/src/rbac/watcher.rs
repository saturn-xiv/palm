use casbin::{CoreApi, Enforcer};
use prost::Message;
use tokio::sync::Mutex;

use super::super::{queue::amqp::Handler as RabbitMqConsumerHandler, Result};
use super::v1::WatcherMessage;

impl RabbitMqConsumerHandler for Mutex<Enforcer> {
    async fn handle(&self, _id: &str, _content_type: &str, payload: &[u8]) -> Result<()> {
        let msg = WatcherMessage::decode(payload)?;
        if msg.by != WatcherMessage::this_id()? {
            let mut it = self.lock().await;
            log::debug!("reload policies");
            it.load_policy().await?;
        }
        Ok(())
    }
}
