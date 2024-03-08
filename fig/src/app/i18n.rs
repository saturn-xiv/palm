use std::path::Path;

use clap::Parser;
use palm::Result;

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Sync {
    #[clap(short, long)]
    pub folder: String,
}

impl Sync {
    pub async fn launch<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}
