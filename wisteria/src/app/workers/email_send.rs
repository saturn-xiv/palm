use std::any::type_name;
use std::{path::Path, sync::Arc, time::Duration};

use hyacinth::{email_v1::Task, flatbuffers_root};
use lettre::{Message, SmtpTransport, Transport};
use portal::{
    Error, Result, is_stopped,
    mailer::Smtp,
    parse_toml,
    queue::{Consumer as QueueConsumer, rabbitmq::Node as RabbitMq},
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

pub async fn start<P: AsRef<Path>>(config: P, interval: Duration) -> Result<()> {
    if is_stopped()? {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let config: Config = parse_toml(config)?;
    let mailer = Arc::new(config.smtp.open()?);

    let queue = type_name::<Task>();
    let client = config.rabbitmq.open().await?;

    loop {
        if let Err(e) = client
            .consume(
                "email-sender",
                queue,
                &Consumer {
                    mailer: mailer.clone(),
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
    smtp: Smtp,
    rabbitmq: RabbitMq,
}

struct Consumer {
    mailer: Arc<SmtpTransport>,
}

impl QueueConsumer for Consumer {
    type Error = Error;
    async fn consume(&self, _id: &str, _content_type: &str, payload: &[u8]) -> Result<()> {
        let task = flatbuffers_root::<Task>(payload)?;
        log::info!("send email({}) to {:?}", task.subject(), task.to());
        let mail = Message::try_from(task)?;
        self.mailer.send(&mail)?;

        Ok(())
    }
}
