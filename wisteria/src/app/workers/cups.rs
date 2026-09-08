use std::any::type_name;
use std::{fs::File, io::prelude::*, path::Path, process::Command, time::Duration};

use hyacinth::{cups_v1::Task, flatbuffers_root};
use portal::{
    Error, Result, is_stopped, parse_toml,
    queue::{
        Consumer as QueueConsumer,
        rabbitmq::{Node as RabbitMq, QueueDeclareOptions},
    },
};
use serde::{Deserialize, Serialize};
use tempfile::tempdir;
use tokio::time::sleep;

pub async fn start<P: AsRef<Path>>(config: P, interval: Duration) -> Result<()> {
    if is_stopped()? {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let config: Config = parse_toml(config)?;

    {
        let out = Command::new("sh").arg("-c").arg("lpstat -p -l").output()?;
        log::info!("printer list: {}", std::str::from_utf8(&out.stdout)?);
    }

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
        if let Err(e) = client.consume("cups", queue, &Consumer {}, interval).await {
            log::error!("{}", e);
        }
        sleep(Duration::from_mins(1)).await;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    rabbitmq: RabbitMq,
}

struct Consumer {}

impl QueueConsumer for Consumer {
    type Error = Error;
    async fn consume(&self, _id: &str, _content_type: &str, payload: &[u8]) -> Result<()> {
        let task = flatbuffers_root::<Task>(payload)?;
        let work_dir = tempdir()?;
        let file = work_dir.path().join(task.name());
        {
            let mut it = File::create_new(&file)?;
            it.write_all(task.document().bytes())?;
        }
        log::info!("print job({}) {}", task.name(), file.display());
        match task.command(&file) {
            Ok(ref cmd) => {
                log::debug!("{}", cmd);
                let out = Command::new("sh").arg("-c").arg(cmd).output()?;
                log::debug!("{}", std::str::from_utf8(&out.stdout)?);
            }
            Err(err) => log::error!("{}", err),
        }

        Ok(())
    }
}
