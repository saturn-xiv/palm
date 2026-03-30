use std::{ops::Deref, sync::Arc};

use phlox::{
    Error, Result,
    queue::{
        Consumer, FlexbuffersConsumer, ProtobufConsumer,
        rabbitmq::{
            BasicPublishOptions, Client, FlexBuffersMessage,
            FlexbuffersConsumer as RabbitMqFlexbufferConsumer, Node,
            ProtobufConsumer as RabbitMqProtobufConsumer, ProtobufMessage, QueueDeclareOptions,
        },
    },
};
use serde::{Deserialize, Serialize};

const PLAIN_CONSUMER_QUEUE: &str = "consumer.plain";
const FLEXBUFFERS_CONSUMER_QUEUE: &str = "consumer.flexbuffers";
const PROTOBUF_CONSUMER_QUEUE: &str = "consumer.protobuf";

const HI: &str = "Hello, Phlox!";

struct EchoConsumer;

impl Consumer for EchoConsumer {
    type Error = Error;
    async fn consume(&self, id: &str, content_type: &str, payload: &[u8]) -> Result<()> {
        let msg = std::str::from_utf8(payload).unwrap();
        println!("({},{}): {}", id, content_type, msg);
        Ok(())
    }
}

struct ProtobufEchoConsumer;

impl ProtobufConsumer for ProtobufEchoConsumer {
    type Message = prost_types::Duration;
    async fn consume(&self, id: &str, task: &Self::Message) -> Result<()> {
        println!("echo(protobuf) {}: duration-{}", id, task.seconds);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EchoMessage {
    text: String,
}
struct FlexbuffersEchoConsumer;

impl FlexbuffersConsumer for FlexbuffersEchoConsumer {
    type Message = EchoMessage;
    async fn consume(&self, id: &str, task: &Self::Message) -> Result<()> {
        println!("echo(flexbuffers) {}: {}", id, task.text);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    text: String,
}

#[tokio::test]
async fn plain_producer() {
    let client = open_rabbitmq().await.unwrap();
    let content_type = mime::TEXT_PLAIN_UTF_8.to_string();
    for i in 1..10 {
        client
            .publish(
                "",
                PLAIN_CONSUMER_QUEUE,
                &content_type,
                format!("{}-{}", i, HI).as_bytes(),
                BasicPublishOptions::default(),
            )
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn flexbuffers_producer() {
    let client = open_rabbitmq().await.unwrap();
    let client = client.deref();
    for i in 1..10 {
        FlexBuffersMessage::publish(
            client,
            "",
            FLEXBUFFERS_CONSUMER_QUEUE,
            &Message {
                text: format!("f {}-{}", i, HI),
            },
            BasicPublishOptions::default(),
        )
        .await
        .unwrap();
    }
}

#[tokio::test]
async fn protobuf_producer() {
    let client = open_rabbitmq().await.unwrap();
    let client = client.deref();

    for i in 1..10 {
        ProtobufMessage::publish(
            client,
            "",
            PROTOBUF_CONSUMER_QUEUE,
            &prost_types::Duration {
                seconds: i,
                nanos: 0,
            },
            BasicPublishOptions::default(),
        )
        .await
        .unwrap();
    }
}

#[tokio::test]
async fn plain_consumer() {
    let client = open_rabbitmq().await.unwrap();
    {
        let mut options = QueueDeclareOptions::default();
        options.auto_delete = true;
        client
            .declare_queue(PLAIN_CONSUMER_QUEUE, options)
            .await
            .unwrap();
    }
    client
        .consume(
            "phlox.testing.consumer.plain",
            PLAIN_CONSUMER_QUEUE,
            &EchoConsumer {},
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn flexbuffers_consumer() {
    let client = open_rabbitmq().await.unwrap();
    {
        let mut options = QueueDeclareOptions::default();
        options.auto_delete = true;
        client
            .declare_queue(FLEXBUFFERS_CONSUMER_QUEUE, options)
            .await
            .unwrap();
    }

    client
        .consume(
            "phlox.testing.consumer.flexbuffers",
            FLEXBUFFERS_CONSUMER_QUEUE,
            &RabbitMqFlexbufferConsumer {
                handler: FlexbuffersEchoConsumer {},
            },
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn protobuf_consumer() {
    let client = open_rabbitmq().await.unwrap();
    {
        let mut options = QueueDeclareOptions::default();
        options.auto_delete = true;
        client
            .declare_queue(PROTOBUF_CONSUMER_QUEUE, options)
            .await
            .unwrap();
    }

    client
        .consume(
            "phlox.testing.consumer.protobuf",
            PROTOBUF_CONSUMER_QUEUE,
            &RabbitMqProtobufConsumer {
                handler: ProtobufEchoConsumer {},
            },
        )
        .await
        .unwrap();
}

async fn open_rabbitmq() -> Result<Arc<Client>> {
    let cfg = Node {
        host: "127.0.0.1".to_string(),
        port: 5672,
        user: "www".to_string(),
        password: "change-me".to_string(),
        virtual_host: "testing".to_string(),
    };

    let it = cfg.open().await?;
    Ok(it)
}
