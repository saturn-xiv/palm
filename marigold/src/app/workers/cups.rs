use std::{fs::File, io::prelude::*, path::Path, process::Command, time::Duration};

use phlox::{
    Result, is_stopped, parse_toml,
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

use super::super::super::palm::cups::v1::{
    Task,
    task::{JobSheet, Media, Orientation, Quality, Sides},
};

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
                "phlox.cups",
                queue,
                &RabbitMqProtobufConsumer {
                    handler: ProtobufCupsConsumer { interval },
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
    rabbitmq: RabbitMq,
}

struct ProtobufCupsConsumer {
    interval: Duration,
}

impl ProtobufConsumer for ProtobufCupsConsumer {
    type Message = Task;
    async fn consume(&self, _id: &str, task: Self::Message) -> Result<()> {
        let work_dir = tempdir()?;
        let file = work_dir.path().join(&task.name);
        {
            let mut it = File::create_new(&file)?;
            it.write_all(&task.document)?;
        }
        log::info!("print job({}) {}", task.name, file.display());
        {
            let cmd = task.shell_command(&file)?;
            log::debug!("{}", cmd);
            let out = Command::new("sh").arg("-c").arg(&cmd).output()?;
            log::debug!("{}", std::str::from_utf8(&out.stdout)?);
        }
        sleep(self.interval).await;
        Ok(())
    }
}

impl Task {
    // https://man7.org/linux/man-pages/man1/lpr.1.html
    pub fn shell_command<P: AsRef<Path>>(&self, file: P) -> Result<String> {
        let file = file.as_ref();
        let mut it = format!("lpr -T {} -#{} -r", self.name, self.copies);

        if !self.number_up.is_empty() {
            let pages: Vec<String> = self.number_up.iter().map(|x| format!("{}", x)).collect();
            it = format!("{} -o number-up={}", it, pages.join("|"));
        }

        it = format!(
            "{} -o media={}",
            it,
            match Media::try_from(self.media)? {
                Media::A3 => "a3",
                Media::A4 => "a4",
                Media::Letter => "letter",
            }
        );
        it = format!(
            "{} -o job-sheets={}",
            it,
            match JobSheet::try_from(self.job_sheet)? {
                JobSheet::Classified => "classified",
                JobSheet::Confidential => "confidential",
                JobSheet::Secret => "secret",
                JobSheet::Standard => "standard",
                JobSheet::TopSecret => "topsecret",
                JobSheet::Unclassified => "unclassified",
            }
        );
        it = format!(
            "{} -o orientation-requested={}",
            it,
            match Orientation::try_from(self.orientation)? {
                Orientation::LandscapeCounterClockwise90 => 4,
                Orientation::LandscapeClockwise90 => 5,
                Orientation::ReversePortrait => 6,
            }
        );
        it = format!(
            "{} -o print-quality={}",
            it,
            match Quality::try_from(self.quality)? {
                Quality::Draft => 3,
                Quality::Normal => 4,
                Quality::Best => 5,
            }
        );
        it = format!(
            "{} -o sides={}",
            it,
            match Sides::try_from(self.sides)? {
                Sides::One => "one-sided",
                Sides::TwoLong => "two-sided-long-edge",
                Sides::TwoShort => "two-sided-short-edge",
            }
        );

        Ok(format!("{} {}", it, file.display()))
    }
}
