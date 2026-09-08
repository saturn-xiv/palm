use std::any::type_name;
use std::{path::Path, sync::Arc, time::Duration};

use hyacinth::{flatbuffers_root, sms_v1::Task};
use portal::{
    Error, Result, is_stopped, parse_toml,
    queue::{
        Consumer as QueueConsumer,
        rabbitmq::{Node as RabbitMq, QueueDeclareOptions},
    },
    twilio::Node as TwilioConfig,
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

pub async fn start<P: AsRef<Path>>(config: P, interval: Duration) -> Result<()> {
    if is_stopped()? {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let config: Config = parse_toml(config)?;
    let twilio = Arc::new(config.twikio);

    let queue = type_name::<Task>();
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
                "sms-sender",
                queue,
                &Consumer {
                    client: twilio.clone(),
                },
                interval,
            )
            .await
        {
            log::error!("{}", e);
        }
        sleep(Duration::from_mins(1)).await;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    twikio: TwilioConfig,
    rabbitmq: RabbitMq,
}

struct Consumer {
    client: Arc<TwilioConfig>,
}

impl QueueConsumer for Consumer {
    type Error = Error;
    async fn consume(&self, _id: &str, _content_type: &str, payload: &[u8]) -> Result<()> {
        let task = flatbuffers_root::<Task>(payload)?;
        for to in task.to().iter() {
            self.client
                .sms(
                    to.to_string(),
                    task.body().to_string(),
                    task.status_callback().map(|x| x.to_string()),
                )
                .await?;
        }
        Ok(())
    }
}
