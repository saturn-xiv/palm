use std::any::type_name;
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};

use amqprs::{
    callbacks::DefaultChannelCallback,
    channel::{
        BasicAckArguments, BasicConsumeArguments, BasicPublishArguments, Channel,
        ExchangeDeclareArguments, ExchangeType, QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};
use async_trait::async_trait;
use hyper::StatusCode;
use serde::Serialize;
use uuid::Uuid;

use super::super::super::{HttpError, Result, FLATBUFFER, PROTOBUF};

#[async_trait]
pub trait Handler: Sync + Send {
    async fn handle(&self, message_id: &str, content_type: &str, payload: &[u8]) -> Result<()>;
}

pub trait Amqp {
    fn open(&self) -> impl std::future::Future<Output = Result<RabbitMq>> + Send;
}

impl Amqp for super::Config {
    async fn open(&self) -> Result<RabbitMq> {
        let con = Connection::open(
            &OpenConnectionArguments::new(&self.host, self.port, &self.user, &self.password)
                .virtual_host(&self.virtual_host)
                .finish(),
        )
        .await?;
        let ch = {
            let it = con.open_channel(None).await?;
            it.register_callback(DefaultChannelCallback).await?;
            it
        };
        Ok(RabbitMq { channel: ch })
    }
}

pub struct RabbitMq {
    channel: Channel,
}

impl RabbitMq {
    pub async fn declare_queue(&self, name: &str) -> Result<()> {
        self.channel
            .queue_declare(
                QueueDeclareArguments::default()
                    .queue(name.to_string())
                    .auto_delete(false)
                    .durable(true)
                    .finish(),
            )
            .await?;

        Ok(())
    }
    pub async fn declare_exchange(&self, name: &str, type_: ExchangeType) -> Result<()> {
        self.channel
            .exchange_declare(
                ExchangeDeclareArguments::of_type(name, type_)
                    .durable(true)
                    .auto_delete(false)
                    .finish(),
            )
            .await?;
        Ok(())
    }
    pub async fn bind(&self, exchange: &str, queue: &str, routing_key: &str) -> Result<()> {
        self.channel
            .queue_bind(QueueBindArguments::new(queue, exchange, routing_key))
            .await?;
        Ok(())
    }
}

pub trait Protobuf {
    fn produce<T: prost::Message>(
        &self,
        task: &T,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn publish<T: prost::Message>(
        &self,
        task: &T,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Protobuf for RabbitMq {
    async fn produce<T: prost::Message>(&self, task: &T) -> Result<()> {
        let payload = task.encode_to_vec();
        self.produce(type_name::<T>(), PROTOBUF, &payload).await
    }
    async fn publish<T: prost::Message>(&self, task: &T) -> Result<()> {
        let payload = task.encode_to_vec();
        self.publish(type_name::<T>(), PROTOBUF, &payload).await
    }
}

pub trait Flatbuffer {
    fn produce<T: Serialize + Debug + Send + Sync>(
        &self,
        task: &T,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn publish<T: Serialize + Debug + Send + Sync>(
        &self,
        task: &T,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Flatbuffer for RabbitMq {
    async fn produce<T: Serialize + Debug + Send + Sync>(&self, task: &T) -> Result<()> {
        let payload = flexbuffers::to_vec(task)?;
        self.produce(type_name::<T>(), FLATBUFFER, &payload).await
    }
    async fn publish<T: Serialize + Debug + Send + Sync>(&self, task: &T) -> Result<()> {
        let payload = flexbuffers::to_vec(task)?;
        self.publish(type_name::<T>(), FLATBUFFER, &payload).await
    }
}

impl RabbitMq {
    // https://www.rabbitmq.com/tutorials/tutorial-three-python.html
    pub async fn publish(&self, queue: &str, content_type: &str, task: &[u8]) -> Result<()> {
        self.send(queue, "", content_type, task).await
    }
    // https://www.rabbitmq.com/tutorials/tutorial-two-python.html
    pub async fn produce(&self, queue: &str, content_type: &str, task: &[u8]) -> Result<()> {
        self.send("", queue, content_type, task).await
    }
    async fn send(
        &self,
        exchange: &str,
        routing_key: &str,
        content_type: &str,
        payload: &[u8],
    ) -> Result<()> {
        let message_id = Uuid::new_v4().to_string();
        debug!("send message {message_id}@({exchange}, {routing_key})");
        self.channel
            .basic_publish(
                BasicProperties::default()
                    .with_message_id(&message_id)
                    .with_content_type(content_type)
                    .with_timestamp(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
                    .finish(),
                payload.to_vec(),
                BasicPublishArguments::new(exchange, routing_key),
            )
            .await?;
        Ok(())
    }

    pub async fn consume(
        &self,
        consumer: &str,
        queue: &str,
        handler: Box<dyn Handler>,
        exclusive: bool,
    ) -> Result<()> {
        self.channel
            .basic_consume(
                Consumer { handler },
                BasicConsumeArguments::new(queue, consumer)
                    .manual_ack(true)
                    .exclusive(exclusive)
                    .finish(),
            )
            .await?;
        Ok(())
    }
}

pub struct Consumer {
    handler: Box<dyn Handler>,
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
            error!("{:?}", e);
        }
    }
}

impl Consumer {
    async fn handle(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        properties: BasicProperties,
        content: Vec<u8>,
    ) -> Result<()> {
        debug!(
            "receive message delivery {} on channel {}, content size: {}",
            deliver,
            channel,
            content.len()
        );
        let message_id = properties.message_id().ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("empty message id".to_string()),
        )))?;
        let content_type = properties.content_type().ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("empty content type".to_string()),
        )))?;
        info!("receive message {message_id},{content_type}");
        self.handler
            .handle(message_id, content_type, &content)
            .await?;
        info!("ack to delivery {} on channel {}", deliver, channel);
        channel
            .basic_ack(BasicAckArguments::new(deliver.delivery_tag(), false))
            .await?;

        Ok(())
    }
}
