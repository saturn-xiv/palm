use std::{path::Path, sync::Arc, time::Duration};

use phlox::{
    Result, is_stopped, parse_toml,
    queue::{
        ProtobufConsumer,
        rabbitmq::{
            Node as RabbitMq, ProtobufConsumer as RabbitMqProtobufConsumer, QueueDeclareOptions,
        },
    },
    twilio::Node as TwilioConfig,
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use super::super::super::palm::sms::v1::Task;

pub async fn start<P: AsRef<Path>>(config: P, queue: &str, interval: Duration) -> Result<()> {
    if is_stopped() {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let config: Config = parse_toml(config)?;
    let twilio = Arc::new(config.twikio);

    let client = config.rabbitmq.open().await?;
    client
        .declare_queue(
            queue,
            QueueDeclareOptions {
                durable: true,
                exclusive: true,
                ..Default::default()
            },
        )
        .await?;

    loop {
        if let Err(e) = client
            .consume(
                "phlox.email-send",
                queue,
                &RabbitMqProtobufConsumer {
                    handler: ProtobufSmsSendConsumer {
                        client: twilio.clone(),
                        interval,
                    },
                },
            )
            .await
        {
            log::error!("{}", e);
        }
        sleep(Duration::from_mins(1)).await;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    twikio: TwilioConfig,
    rabbitmq: RabbitMq,
}

struct ProtobufSmsSendConsumer {
    client: Arc<TwilioConfig>,
    interval: Duration,
}

impl ProtobufConsumer for ProtobufSmsSendConsumer {
    type Message = Task;
    async fn consume(&self, _id: &str, task: Self::Message) -> Result<()> {
        for to in task.to.iter() {
            self.client
                .sms(to.clone(), task.body.clone(), task.status_callback.clone())
                .await?;
            sleep(self.interval).await;
        }
        Ok(())
    }
}
