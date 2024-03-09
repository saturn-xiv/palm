use std::ops::Deref;
use std::sync::Arc;

use amqprs::{
    channel::{BasicAckArguments, Channel, ExchangeType},
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};
use async_trait::async_trait;
use casbin::{CoreApi, Enforcer, EventData, Watcher as CasbinWatcher};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use super::super::super::super::{HttpError, Result};
use super::{Flatbuffer, RabbitMq};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncTask;

pub struct Watcher {
    rabbitmq: Arc<RabbitMq>,
    callback: Box<dyn FnMut() + Send + Sync>,
    pub queue: String,
}

impl Watcher {
    const EXCHANGE_NAME: &'static str = "ex.casbin";
    pub async fn new(rabbitmq: Arc<RabbitMq>) -> Result<Self> {
        rabbitmq
            .declare_exchange(Self::EXCHANGE_NAME, ExchangeType::Fanout)
            .await?;
        let queue = rabbitmq.declare_queue("", false).await?;
        info!("casbin watcher use queue {queue}");
        rabbitmq.bind(Self::EXCHANGE_NAME, &queue, "").await?;

        Ok(Self {
            rabbitmq,
            queue,
            callback: Box::new(|| {
                debug!("empty casbin callback");
            }),
        })
    }
}

impl CasbinWatcher for Watcher {
    fn set_update_callback(&mut self, callback: Box<dyn FnMut() + Send + Sync>) {
        debug!("update casbin watcher callback");
        self.callback = callback;
    }
    fn update(&mut self, event: EventData) {
        debug!("casbin watcher receive event: {}", event);
        let queue = self.rabbitmq.clone();
        tokio::spawn(async move {
            let queue = queue.deref();
            if let Err(e) = Flatbuffer::publish(queue, &SyncTask).await {
                error!("{:?}", e);
            }
        });
    }
}

pub async fn consume(
    name: &str,
    rabbitmq: &RabbitMq,
    queue: &str,
    enforcer: Arc<Mutex<Enforcer>>,
) -> Result<()> {
    rabbitmq
        .consume(name, queue, Box::new(Handler { enforcer }), true)
        .await
}

struct Handler {
    enforcer: Arc<Mutex<Enforcer>>,
}

#[async_trait]
impl super::Handler for Handler {
    async fn handle(&self, id: &str, _content_type: &str, _payload: &[u8]) -> Result<()> {
        debug!("reload casbin policies message {}", id);
        let mut enforcer = self.enforcer.lock().await;
        enforcer.load_policy().await?;
        Ok(())
    }
}

struct Consumer {
    enforcer: Arc<Mutex<Enforcer>>,
}

#[async_trait]
impl AsyncConsumer for Consumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        if let Err(e) = self
            .handle(channel, deliver, _basic_properties, content)
            .await
        {
            error!("handle casbin watch message {:?}", e);
        }
    }
}

impl Consumer {
    async fn handle(
        &self,
        channel: &Channel,
        deliver: Deliver,
        properties: BasicProperties,
        content: Vec<u8>,
    ) -> Result<()> {
        let message_id = properties.message_id().ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("empty message id".to_string()),
        )))?;
        info!("reload casbin policies message {}", message_id);

        {
            let _: SyncTask = flexbuffers::from_slice(&content)?;
            let mut enforcer = self.enforcer.lock().await;
            enforcer.load_policy().await?;
        }

        info!("ack to delivery {} on channel {}", deliver, channel);
        channel
            .basic_ack(BasicAckArguments::new(deliver.delivery_tag(), false))
            .await?;
        Ok(())
    }
}
