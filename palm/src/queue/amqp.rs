use std::any::type_name;
use std::fmt::Debug;
use std::ops::Deref;

use futures::StreamExt;
use hyper::StatusCode;
use rabbitmq_stream_client::{
    types::{Message, OffsetSpecification},
    Environment,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::{is_stopped, HttpError, Result, FLATBUFFER, PROTOBUF};

pub trait Handler: Sync + Send {
    fn handle(
        &self,
        content_type: &str,
        payload: &[u8],
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    #[serde(rename = "virtual-host")]
    pub virtual_host: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5672,
            user: "guest".to_string(),
            password: "guest".to_string(),
            virtual_host: "dev".to_string(),
        }
    }
}

impl Config {
    pub async fn open(&self) -> Result<Environment> {
        let it = Environment::builder()
            .host(&self.host)
            .port(self.port)
            .username(&self.user)
            .password(&self.password)
            .build()
            .await
            .map_err(|x| {
                Box::new(HttpError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(x.to_string()),
                ))
            })?;

        Ok(it)
    }
}

#[derive(Clone)]
pub struct RabbitMq {
    pub name: String,
    pub environment: Environment,
}

pub trait Protobuf {
    fn publish<T: prost::Message>(
        &self,
        task: &T,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Protobuf for RabbitMq {
    async fn publish<T: prost::Message>(&self, task: &T) -> Result<()> {
        let mut payload = Vec::new();
        task.encode(&mut payload)?;
        self.publish(type_name::<T>(), PROTOBUF, &payload).await
    }
}

pub trait Flatbuffer {
    fn publish<T: Serialize + Debug + Send + Sync>(
        &self,
        task: &T,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Flatbuffer for RabbitMq {
    async fn publish<T: Serialize + Debug + Send + Sync>(&self, task: &T) -> Result<()> {
        let payload = flexbuffers::to_vec(task)?;
        self.publish(type_name::<T>(), FLATBUFFER, &payload).await
    }
}

impl RabbitMq {
    // https://www.rabbitmq.com/tutorials/tutorial-three-python.html
    pub async fn publish(&self, queue: &str, content_type: &str, task: &[u8]) -> Result<()> {
        let mut producer = self
            .environment
            .producer()
            .name(&self.name)
            .build(queue)
            .await?;
        let id = Uuid::new_v4().to_string();
        info!("publish message {}/{} into {}", id, content_type, queue);
        let message = Message::builder()
            .properties()
            .message_id(id)
            .content_type(content_type)
            .message_builder()
            .body(task)
            .build();
        producer.send_with_confirm(message).await?;
        Ok(())
    }

    pub async fn consume<H: Handler>(self, queue: &str, handler: &H) -> Result<()> {
        let mut consumer = self
            .environment
            .consumer()
            .name(&self.name)
            .offset(OffsetSpecification::First)
            .build(queue)
            .await?;

        loop {
            info!("waiting for next message...");
            if let Some(delivery) = consumer.next().await {
                let delivery = delivery?;
                if let Some(payload) = delivery.message().data() {
                    if let Some(properties) = delivery.message().properties() {
                        if let Some(ref message_id) = properties.message_id {
                            if let Some(ref content_type) = properties.content_type {
                                let content_type = content_type.deref();
                                info!(
                                    "got message: {:?}/{} with offset: {}",
                                    message_id,
                                    content_type,
                                    delivery.offset(),
                                );
                                handler.handle(content_type, payload).await?;
                                if is_stopped() {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        consumer.handle().close().await?;

        Ok(())
    }
}
