use std::{fs::File, io::prelude::*, path::Path, process::Command, sync::Arc, time::Duration};

use hyper::StatusCode;
use phlox::{
    HttpError, Result, is_stopped,
    minio::{Client as MinioClient, Node as MinioConfig},
    parse_toml,
    queue::{
        ProtobufConsumer,
        rabbitmq::{
            Node as RabbitMq, ProtobufConsumer as RabbitMqProtobufConsumer, QueueDeclareOptions,
        },
    },
    tempdir,
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use super::super::super::palm::tex::v1::Task;

pub async fn start<P: AsRef<Path>>(config: P, queue: &str, interval: Duration) -> Result<()> {
    if is_stopped() {
        log::warn!("stopped file exists, exit...");
        return Ok(());
    }
    let config: Config = parse_toml(config)?;

    {
        let out = Command::new("sh").arg("-c").arg("lpstat -p -l").output()?;
        log::info!("printer list: {}", std::str::from_utf8(&out.stdout)?);
    }

    let s3 = Arc::new(config.minio.open()?);
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
                "phlox.tex",
                queue,
                &RabbitMqProtobufConsumer {
                    handler: ProtobuTexConsumer {
                        interval,
                        s3: s3.clone(),
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
    rabbitmq: RabbitMq,
    minio: MinioConfig,
}

struct ProtobuTexConsumer {
    interval: Duration,
    s3: Arc<MinioClient>,
}

impl ProtobufConsumer for ProtobuTexConsumer {
    type Message = Task;
    async fn consume(&self, _id: &str, task: Self::Message) -> Result<()> {
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
                it.write_all(task.entry.as_bytes())?;
            }
            for (name, body) in task.files.iter() {
                log::debug!("create file {}", name);
                let mut it = File::create_new(work_dir.join(name))?;
                it.write_all(body)?;
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
            let target = task.target.ok_or_else(|| {
                HttpError(StatusCode::BAD_REQUEST, Some("empty target".to_string()))
            })?;
            self.s3
                .upload(&target.bucket, &target.object, &work_dir.join(&entry_pdf))
                .await?;
        }
        sleep(self.interval).await;
        Ok(())
    }
}
