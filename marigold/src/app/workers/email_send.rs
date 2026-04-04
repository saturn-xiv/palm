use std::{path::Path, result::Result as StdResult, sync::Arc, time::Duration};

use hyper::StatusCode;
use phlox::{
    Error, HttpError, Result, is_stopped,
    mailer::{
        AddressError, Attachment, ContentType, Mailbox, Message, MultiPart, SinglePart, Smtp,
        SmtpTransport, Transport,
    },
    parse_toml,
    queue::{
        ProtobufConsumer,
        rabbitmq::{
            Node as RabbitMq, ProtobufConsumer as RabbitMqProtobufConsumer, QueueDeclareOptions,
        },
    },
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use super::super::super::palm::email::v1::{Task, task::Address};

pub async fn start<P: AsRef<Path>>(config: P, queue: &str, interval: Duration) -> Result<()> {
    if is_stopped() {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let config: Config = parse_toml(config)?;
    let mailer = Arc::new(config.smtp.open()?);

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

    let from = Mailbox {
        name: None,
        email: config.smtp.user.parse()?,
    };
    loop {
        if let Err(e) = client
            .consume(
                "phlox.email-send",
                queue,
                &RabbitMqProtobufConsumer {
                    handler: ProtobufEmailSendConsumer {
                        mailer: mailer.clone(),
                        interval,
                        from: from.clone(),
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
struct Config {
    smtp: Smtp,
    rabbitmq: RabbitMq,
}

struct ProtobufEmailSendConsumer {
    mailer: Arc<SmtpTransport>,
    interval: Duration,
    from: Mailbox,
}

impl ProtobufConsumer for ProtobufEmailSendConsumer {
    type Message = Task;
    async fn consume(&self, _id: &str, task: Self::Message) -> Result<()> {
        log::info!("send email({}) to {:?}", task.subject, task.to);
        let mail = Message::try_from(Job {
            task,
            from: self.from.clone(),
        })?;
        self.mailer.send(&mail)?;
        sleep(self.interval).await;
        Ok(())
    }
}

struct Job {
    task: Task,
    from: Mailbox,
}

impl TryFrom<Job> for Message {
    type Error = Error;

    fn try_from(job: Job) -> StdResult<Self, Self::Error> {
        let mut builder = Message::builder()
            .subject(&job.task.subject)
            .from(job.from.clone());

        if let Some(ref it) = job.task.reply_to {
            builder = builder.reply_to(Mailbox::try_from(it.clone())?);
        }
        {
            let it = job.task.to.as_ref().ok_or_else(|| {
                HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("empty to address".to_string()),
                )
            })?;
            builder = builder.to(Mailbox::try_from(it.clone())?);
        }
        for it in job.task.cc.iter() {
            builder = builder.cc(Mailbox::try_from(it.clone())?);
        }
        for it in job.task.bcc.iter() {
            builder = builder.bcc(Mailbox::try_from(it.clone())?);
        }

        let mut parts = {
            let body = job.task.body.as_ref().ok_or_else(|| {
                HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("empty email body".to_string()),
                )
            })?;
            MultiPart::mixed().singlepart(if body.html {
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body.content.clone())
            } else {
                SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(body.content.clone())
            })
        };

        for it in job.task.attachments.iter() {
            let part = match it.inline_id {
                // <img src="cid:123">
                Some(ref content_id) => {
                    Attachment::new_inline_with_name(content_id.clone(), it.name.clone())
                        .body(it.body.clone(), ContentType::parse(&it.content_type)?)
                }
                None => Attachment::new(it.name.clone())
                    .body(it.body.clone(), ContentType::parse(&it.content_type)?),
            };
            parts = parts.singlepart(part);
        }

        Ok(builder.multipart(parts)?)
    }
}

impl TryFrom<Address> for Mailbox {
    type Error = AddressError;

    fn try_from(it: Address) -> StdResult<Self, Self::Error> {
        Ok(Self {
            name: Some(it.name),
            email: it.email.parse()?,
        })
    }
}
