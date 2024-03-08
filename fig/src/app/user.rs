use std::path::Path;

use clap::Parser;
use palm::Result;

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Create {
    #[clap(short, long)]
    pub nickname: String,
    #[clap(short, long)]
    pub email: String,
    #[clap(short, long)]
    pub password: String,
}

impl Create {
    pub async fn launch<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Token {
    #[clap(short, long)]
    pub user: String,
    #[clap(short, long, default_value_t = 1<<12)]
    pub weeks: u32,
}

impl Token {
    pub async fn launch<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Role {
    #[clap(short, long)]
    pub user: String,
    #[clap(short, long)]
    pub role: String,
}

impl Role {
    pub async fn apply<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
    pub async fn exempt<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct ResetPassword {
    #[clap(short, long)]
    pub user: String,
    #[clap(short, long)]
    pub password: String,
}

impl ResetPassword {
    pub async fn launch<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}

pub async fn list<P: AsRef<Path>>(_config_file: P) -> Result<()> {
    // TODO
    Ok(())
}
