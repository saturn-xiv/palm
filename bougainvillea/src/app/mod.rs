pub mod token;
pub mod web;

use std::ops::Deref;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::info;
use palm::{crypto::Key, env::Environment, parser::from_toml, Result, BANNER, HOMEPAGE, VERSION};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(about, version=&VERSION.deref()[..], before_help=BANNER, after_help=HOMEPAGE, author)]
pub struct Args {
    #[clap(short, long, default_value = "config.toml")]
    pub config: PathBuf,
    #[clap(subcommand)]
    pub command: SubCommand,
}

pub async fn launch() -> Result<()> {
    let args = Args::parse();

    let config: Config = {
        info!("load config from {}", args.config.display());
        palm::check_config_permission(&args.config)?;
        from_toml(&args.config)?
    };

    if let SubCommand::GenerateToken(ref token) = args.command {
        return token.generate(&config);
    }
    if let SubCommand::Web(ref web) = args.command {
        return web.launch(&config).await;
    }
    Ok(())
}

#[derive(Subcommand, PartialEq, Eq, Debug)]
pub enum SubCommand {
    #[clap(about = "Generate a token")]
    GenerateToken(token::Generate),
    #[clap(about = "Http Server")]
    Web(web::Server),
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub env: Environment,
    // openssl rand -base64 128
    #[serde(rename = "cookie-key")]
    pub cookie_key: Key,
    // openssl rand -base64 32
    #[serde(rename = "secret-key")]
    pub secret_key: Key,
}
