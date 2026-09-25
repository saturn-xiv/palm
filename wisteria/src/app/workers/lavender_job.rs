use std::any::type_name;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use diesel::Connection as DieselConnection;
use lavender::{
    Config as Lavender,
    models::{
        job::{Item as Job, Message},
        task::{Dao as TaskDao, Output},
    },
};
use portal::{
    Error, Result, is_stopped,
    mailer::Smtp,
    orm::postgresql::{Node as PostgreSql, Pool as DbPool},
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
    let db = config.postgresql.open()?;
    let lavender = Arc::new(config.lavender);

    let queue = type_name::<Message>();
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
                    db: db.clone(),
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
    postgresql: PostgreSql,
}

struct Consumer {
    config: Arc<Lavender>,
    queue: QueueClient,
    db: DbPool,
    from: String,
}

impl QueueConsumer for Consumer {
    type Error = Error;
    async fn consume(&self, _id: &str, _content_type: &str, payload: &[u8]) -> Result<()> {
        let start = Instant::now();
        let task: Message = flexbuffers::from_slice(payload)?;
        let job = Job::new(&self.config.jobs_dir, &task.name)?;
        let output = job.execute(&self.config.working_dir, task.args)?;
        let duration = start.elapsed();
        let output = Output::new(&output, duration)?;
        {
            let mut db = self.db.get()?;
            let db = db.deref_mut();

            db.transaction::<_, Error, _>(|tx| {
                let it = TaskDao::by_uid(tx, &task.uid)?;
                TaskDao::set_output(tx, it.id, &output)?;
                Ok(())
            })?;
        }

        job.report(
            &task.name,
            &self.queue,
            &self.from,
            &task.email,
            self.config.bcc.clone(),
            if let Some(0) = output.code {
                (&output.stdout, true, duration)
            } else {
                (&output.stderr, false, duration)
            },
        )
        .await?;

        Ok(())
    }
}
