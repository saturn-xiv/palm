use std::any::type_name;
use std::fmt::Debug;
use std::ops::Deref;
use std::time::Duration as StdDuration;

use futures::StreamExt;
use hyper::StatusCode;
use palm::{random::uuid, HttpError, Result, FLATBUFFER, PROTOBUF};
use rabbitmq_stream_client::{
    error::StreamCreateError,
    types::{ByteCapacity, Message, OffsetSpecification, ResponseCode},
    Environment,
};
use serde::Serialize;

use super::super::super::is_stopped;

pub trait Handler: Sync + Send {
    fn handle(
        &self,
        message_id: &str,
        content_type: &str,
        payload: &[u8],
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

pub trait Stream {
    fn open(&self, name: &str) -> impl std::future::Future<Output = Result<Client>> + Send;
}

impl Stream for super::Config {
    async fn open(&self, name: &str) -> Result<Client> {
        debug!(
            "open rabbitmq-stream://{}@{}:{}/{}",
            self.user, self.host, self.port, self.virtual_host
        );
        let environment = Environment::builder()
            .host(&self.host)
            .port(self.port)
            .username(&self.user)
            .password(&self.password)
            .virtual_host(&self.virtual_host)
            .heartbeat(20)
            .build()
            .await
            .map_err(|x| {
                Box::new(HttpError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(format!("open rabbitmq stream: {x:?}")),
                ))
            })?;

        Ok(Client {
            name: name.to_string(),
            environment,
        })
    }
}

pub struct Client {
    pub name: String,
    pub environment: Environment,
}

pub trait Protobuf {
    fn produce<T: prost::Message>(
        &self,
        task: &T,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Protobuf for Client {
    async fn produce<T: prost::Message>(&self, task: &T) -> Result<()> {
        let mut payload = Vec::new();
        task.encode(&mut payload)?;
        self.produce(type_name::<T>(), PROTOBUF, &payload).await
    }
}

pub trait Flatbuffer {
    fn produce<T: Serialize + Debug + Send + Sync>(
        &self,
        task: &T,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Flatbuffer for Client {
    async fn produce<T: Serialize + Debug + Send + Sync>(&self, task: &T) -> Result<()> {
        let payload = flexbuffers::to_vec(task)?;
        self.produce(type_name::<T>(), FLATBUFFER, &payload).await
    }
}

impl Client {
    pub async fn create_stream(
        &self,
        queue: &str,
        capacity_in_gb: u64,
        max_age_in_weeks: u64,
    ) -> Result<()> {
        debug!("create stream {queue}");
        match self
            .environment
            .stream_creator()
            .max_length(ByteCapacity::GB(capacity_in_gb))
            .max_age(StdDuration::from_secs(60 * 60 * 24 * 7 * max_age_in_weeks))
            .create(queue)
            .await
        {
            Ok(_) => Ok(()),
            Err(StreamCreateError::Create {
                stream: _,
                status: ResponseCode::StreamAlreadyExists,
            }) => {
                debug!("stream {queue} already exists!");
                Ok(())
            }
            Err(e) => Err(e),
        }?;
        Ok(())
    }

    pub async fn produce(&self, queue: &str, content_type: &str, task: &[u8]) -> Result<()> {
        let mut producer = self
            .environment
            .producer()
            .name(&self.name)
            .build(queue)
            .await?;
        let id = uuid();
        info!("publish message {}/{} into {}", id, content_type, queue);
        let message = Message::builder()
            .properties()
            .message_id(id)
            .content_type(content_type)
            .message_builder()
            .body(task)
            .build();
        let reply = producer.send_with_confirm(message).await?;
        info!(
            "{:?} {} {}",
            reply.status(),
            reply.publishing_id(),
            reply.confirmed()
        );
        debug!("{:?}", reply.message());
        producer.close().await?;
        Ok(())
    }

    pub async fn consume<H: Handler>(
        &self,
        queue: &str,
        handler: &H,
        offset: OffsetSpecification,
    ) -> Result<()> {
        let mut consumer = self
            .environment
            .consumer()
            .name(&self.name)
            .offset(offset)
            .build(queue)
            .await?;
        info!(
            "waiting for next message({}) on stream({queue})...",
            type_name::<H>(),
        );

        while let Some(delivery) = consumer.next().await {
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
                            handler
                                .handle(&format!("{:?}", message_id), content_type, payload)
                                .await?;
                            if is_stopped() {
                                break;
                            }
                            continue;
                        }
                    }
                }
            }
            warn!("unknown message {delivery:?}");
        }

        warn!(
            "waiting for next message({}) on stream({queue})...",
            type_name::<H>()
        );
        consumer.handle().close().await?;

        Ok(())
    }
}
