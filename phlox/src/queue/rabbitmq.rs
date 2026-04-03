use std::sync::Arc;

use flexbuffers::{FlexbufferSerializer, Reader as FlexbufferReader};
use futures_util::StreamExt;
use hyper::StatusCode;
use lapin::{
    BasicProperties, Connection, ConnectionProperties, Result as LapinResult,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use prost::Message as ProtobufMessage_;
use serde::{Deserialize, Serialize};

pub use lapin::{
    ExchangeKind,
    options::{BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
};

use super::super::{Error, HttpError, Result, content_type::*, random::uuid};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(default = "node_default_host")]
    pub host: String,
    #[serde(default = "node_default_port")]
    pub port: u16,
    #[serde(default = "node_default_user")]
    pub user: String,
    #[serde(default = "node_default_password")]
    pub password: String,
    // https://docs.rs/lapin/4.4.0/lapin/struct.Connection.html#method.connect
    #[serde(rename = "pool-size", default = "node_default_virtual_host")]
    pub virtual_host: String,
}

fn node_default_host() -> String {
    "127.0.0.1".to_string()
}

fn node_default_port() -> u16 {
    5672
}
fn node_default_user() -> String {
    "guest".to_string()
}
fn node_default_password() -> String {
    "guest".to_string()
}
fn node_default_virtual_host() -> String {
    "%2f".to_string()
}
impl Default for Node {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5672,
            user: "guest".to_string(),
            password: "guest".to_string(),
            virtual_host: "%2f".to_string(),
        }
    }
}

impl Node {
    pub async fn open(&self) -> LapinResult<Arc<Client>> {
        log::debug!(
            "open RabbitMQ amqp://{}@{}:{}/{}",
            self.user,
            self.host,
            self.port,
            self.virtual_host
        );
        let con = Connection::connect(
            &format!(
                "amqp://{}:{}@{}:{}/{}",
                self.user, self.password, self.host, self.port, self.virtual_host
            ),
            ConnectionProperties::default(),
        )
        .await?;
        Ok(Arc::new(Client { connection: con }))
    }
}

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn bind_queue(
        &self,
        queue: &str,
        exchange: &str,
        routing_key: &str,
        options: QueueBindOptions,
    ) -> LapinResult<()> {
        log::info!("bind queue {} to exchange {}", queue, exchange);
        let channel = self.connection.create_channel().await?;
        channel
            .queue_bind(
                queue.into(),
                exchange.into(),
                routing_key.into(),
                options,
                FieldTable::default(),
            )
            .await?;
        Ok(())
    }
    pub async fn declare_exchange(
        &self,
        name: &str,
        kind: ExchangeKind,
        options: ExchangeDeclareOptions,
    ) -> LapinResult<()> {
        log::info!("declare exchange {}", name);
        let channel = self.connection.create_channel().await?;
        channel
            .exchange_declare(name.into(), kind, options, FieldTable::default())
            .await?;
        Ok(())
    }
    pub async fn declare_anonymous_queue(
        &self,
        options: QueueDeclareOptions,
    ) -> LapinResult<String> {
        log::info!("declare an anonymous queue");
        let channel = self.connection.create_channel().await?;
        let queue = channel
            .queue_declare("".into(), options, FieldTable::default())
            .await?;

        let name = queue.name();
        log::debug!("queue {} was created", name);
        Ok(name.to_string())
    }
    pub async fn declare_queue(&self, name: &str, options: QueueDeclareOptions) -> LapinResult<()> {
        log::info!("declare queue {}", name);
        let channel = self.connection.create_channel().await?;
        let queue = channel
            .queue_declare(name.into(), options, FieldTable::default())
            .await?;
        log::debug!("queue {} was created", queue.name());
        Ok(())
    }
    pub async fn publish(
        &self,
        exchange: &str,
        routing_key: &str,
        content_type: &str,
        payload: &[u8],
        options: BasicPublishOptions,
    ) -> Result<()> {
        let channel = self.connection.create_channel().await?;
        channel
            .basic_publish(
                exchange.into(),
                routing_key.into(),
                options,
                payload,
                BasicProperties::default()
                    .with_message_id(uuid().into())
                    .with_content_type(content_type.into()),
            )
            .await?
            .await?;
        Ok(())
    }

    pub async fn consume<T: super::Consumer>(
        &self,
        name: &str,
        queue: &str,
        handler: &T,
    ) -> Result<()> {
        log::info!("start consumer {}", name);
        let channel = self.connection.create_channel().await?;

        let mut consumer = channel
            .basic_consume(
                queue.into(),
                name.into(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery?;
            let id = delivery.properties.message_id().as_ref().ok_or_else(|| {
                HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("invalid message-id header".to_string()),
                )
            })?;
            let content_type = delivery.properties.content_type().as_ref().ok_or_else(|| {
                HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("invalid content-type header".to_string()),
                )
            })?;
            log::info!(
                "received message({},{}): {} {}",
                delivery.exchange,
                delivery.routing_key,
                id,
                content_type
            );
            handler
                .consume(id.as_str(), content_type.as_str(), &delivery.data)
                .await
                .map_err(|e| {
                    HttpError(StatusCode::INTERNAL_SERVER_ERROR, Some(format!("{:?}", e)))
                })?;

            delivery.ack(BasicAckOptions::default()).await?;
        }

        Ok(())
    }
}

pub trait ProtobufMessage {
    fn publish<V: ProtobufMessage_>(
        &self,
        exchange: &str,
        routing_key: &str,
        task: &V,
        options: BasicPublishOptions,
    ) -> impl Future<Output = Result<()>> + Send;
}

impl ProtobufMessage for Client {
    async fn publish<V: ProtobufMessage_>(
        &self,
        exchange: &str,
        routing_key: &str,
        task: &V,
        options: BasicPublishOptions,
    ) -> Result<()> {
        let mut buf = Vec::new();
        task.encode(&mut buf)?;
        self.publish(exchange, routing_key, APPLICATION_X_PROTOBUF, &buf, options)
            .await?;
        Ok(())
    }
}

pub trait FlexBuffersMessage {
    fn publish<V: Serialize + Sync>(
        &self,
        exchange: &str,
        routing_key: &str,
        payload: &V,
        options: BasicPublishOptions,
    ) -> impl Future<Output = Result<()>> + Send;
}

impl FlexBuffersMessage for Client {
    async fn publish<V: Serialize + Sync>(
        &self,
        exchange: &str,
        routing_key: &str,
        task: &V,
        options: BasicPublishOptions,
    ) -> Result<()> {
        let mut se = FlexbufferSerializer::new();
        task.serialize(&mut se)?;
        self.publish(
            exchange,
            routing_key,
            APPLICATION_X_FLEXBUFFERS,
            se.view(),
            options,
        )
        .await?;
        Ok(())
    }
}

pub struct ProtobufConsumer<H: super::ProtobufConsumer + Sync> {
    pub handler: H,
}

impl<H: super::ProtobufConsumer> super::Consumer for ProtobufConsumer<H> {
    type Error = Error;
    async fn consume(&self, id: &str, content_type: &str, payload: &[u8]) -> Result<()> {
        if content_type != APPLICATION_X_PROTOBUF {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("not a protobuf message".to_string()),
            )));
        }
        let it = H::Message::decode(payload)?;
        self.handler.consume(id, it).await?;
        Ok(())
    }
}

pub struct FlexbuffersConsumer<H: super::FlexbuffersConsumer + Sync> {
    pub handler: H,
}

impl<H: super::FlexbuffersConsumer> super::Consumer for FlexbuffersConsumer<H> {
    type Error = Error;
    async fn consume(&self, id: &str, content_type: &str, payload: &[u8]) -> Result<()> {
        if content_type != APPLICATION_X_FLEXBUFFERS {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("not a flexbuffers message".to_string()),
            )));
        }
        let reader = FlexbufferReader::get_root(payload)?;
        let it = H::Message::deserialize(reader)?;
        self.handler.consume(id, it).await?;
        Ok(())
    }
}
