use std::any::type_name;
use std::{path::Path, sync::Arc, time::Duration};

use lavender::{Config as Lavender, graphql::job::Task, models::job::Item as Job};
use portal::{
    Error, Result, is_stopped, parse_toml,
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
    lavender: Lavender,
    rabbitmq: RabbitMq,
}

struct Consumer {
    config: Arc<Lavender>,
    queue: QueueClient,
}

impl QueueConsumer for Consumer {
    type Error = Error;
    async fn consume(&self, _id: &str, _content_type: &str, payload: &[u8]) -> Result<()> {
        let task: Task = flexbuffers::from_slice(payload)?;
        let job = Job::new(&self.config.jobs_dir, &task.id)?;
        let result = job.execute(&self.config.working_dir, task.args);
        let succeed = result.is_ok();
        let body = result.unwrap_or_else(|e| e.to_string());
        job.report(
            &self.queue,
            &task.email,
            self.config.bcc.clone(),
            &body,
            succeed,
        )
        .await?;

        Ok(())
    }
}
