use std::path::Path;

use clap::{Parser, ValueEnum};
use palm::Result;

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Consumer {
    #[clap(short, long, help = "Consumer name")]
    pub name: String,
    #[clap(short, long, help = "Job name")]
    pub job: Job,
}

impl Consumer {
    pub async fn launch_send_sms<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
    pub async fn launch_send_email<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}

#[derive(ValueEnum, Default, PartialEq, Eq, Debug, Clone)]
pub enum Job {
    #[default]
    SendSms,
    SendEmail,
}
