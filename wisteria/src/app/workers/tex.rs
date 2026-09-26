use std::any::type_name;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use hyacinth::{flatbuffers_root, tex_v1::Task};
use portal::{
    Error, Result, is_stopped,
    orm::postgresql::Node as PostgreSql,
    parse_toml,
    queue::{
        Consumer as QueueConsumer,
        rabbitmq::{Node as RabbitMq, QueueDeclareOptions},
    },
    s3::{
        Provider as S3Provider,
        seaweedfs::{Client as S3, Config as SeaweedFs},
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

    let db = config.postgresql.open()?;
    let s3 = Arc::new(config.seaweedfs.open(db).await?);
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
            .consume("tex-builder", queue, &Consumer { s3: s3.clone() }, interval)
            .await
        {
            log::error!("{}", e);
        }
        sleep(Duration::from_mins(1)).await;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    postgresql: PostgreSql,
    rabbitmq: RabbitMq,
    seaweedfs: SeaweedFs,
}

struct Consumer {
    s3: Arc<S3>,
}

impl QueueConsumer for Consumer {
    type Error = Error;
    async fn consume(&self, _id: &str, _content_type: &str, payload: &[u8]) -> Result<()> {
        let task = flatbuffers_root::<Task>(payload)?;
        let work_dir = tempdir()?;
        let work_dir = work_dir.path();
        let (entry_tex, entry_pdf) = {
            let it = "main";
            (format!("{it}.tex"), format!("{it}.pdf"))
        };

        {
            {
                log::debug!("create entry file");
                let mut it = File::create_new(work_dir.join(&entry_tex))?;
                it.write_all(task.entry().as_bytes())?;
            }
            for file in task.files().iter() {
                log::debug!("create file {}", file.name());
                let mut it = File::create_new(work_dir.join(file.name()))?;
                it.write_all(file.content().bytes())?;
            }
        }

        // https://www.tug.org/levels.html
        // https://tug.ctan.org/macros/latex/contrib/beamer/doc/beameruserguide.pdf
        // https://en.wikibooks.org/wiki/LaTeX/Document_Structure#Document_classes
        log::info!("building {} by TeX Live", work_dir.display());
        for _ in 1..=3 {
            let cmd = format!("lualatex --halt-on-error {}", entry_tex);
            log::debug!("{}", cmd);
            let out = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .current_dir(work_dir)
                .output()?;
            log::debug!("{}", std::str::from_utf8(&out.stdout)?);
        }
        {
            let s3 = self.s3.deref();
            S3Provider::upload(
                s3,
                &work_dir.join(&entry_pdf),
                task.output().bucket(),
                task.output().object(),
            )
            .await?;
        }

        Ok(())
    }
}
