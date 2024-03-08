use std::path::Path;

use clap::Parser;
use palm::Result;

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Server {
    #[clap(short, long)]
    pub port: u16,
    #[clap(short, long)]
    pub threads: usize,
}

impl Server {
    pub async fn launch<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}
