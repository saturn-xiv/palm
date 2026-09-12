use std::any::type_name;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use lavender::{Config as Lavender, graphql::job::Task, models::job::Item as Job};
use portal::{
    Error, Result, is_stopped,
    mailer::Smtp,
    parse_toml,
    queue::{
        Consumer as QueueConsumer,
        rabbitmq::{Client as QueueClient, Node as RabbitMq, QueueDeclareOptions},
    },
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

pub async fn start<P: AsRef<Path>>(config: P, interval: Duration) -> Result<()> {
    if is_stopped()? {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let config: Config = parse_toml(config)?;
    let lavender = Arc::new(config.lavender);

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
                "lavender-job-executer",
                queue,
                &Consumer {
                    queue: config.rabbitmq.open().await?,
                    config: lavender.clone(),
                    from: config.smtp.user.clone(),
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
    lavender: Lavender,
    rabbitmq: RabbitMq,
}

struct Consumer {
    config: Arc<Lavender>,
    queue: QueueClient,
    from: String,
}

impl QueueConsumer for Consumer {
    type Error = Error;
    async fn consume(&self, _id: &str, _content_type: &str, payload: &[u8]) -> Result<()> {
        let start = Instant::now();
        let task: Task = flexbuffers::from_slice(payload)?;
        let job = Job::new(&self.config.jobs_dir, &task.id)?;
        let result = job.execute(&self.config.working_dir, task.args);
        let succeed = result.is_ok();
        let body = result.unwrap_or_else(|e| e.to_string());
        let duration = start.elapsed();
        job.report(
            &self.queue,
            &self.from,
            &task.email,
            self.config.bcc.clone(),
            (&body, succeed, duration),
        )
        .await?;

        Ok(())
    }
}
