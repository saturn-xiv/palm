pub mod nginx;
pub mod systemd;
pub mod token;
pub mod web;

use std::ops::Deref;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use petunia::{check_config_permission, parser::from_toml, Result, BANNER, HOMEPAGE, VERSION};

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Parser, Debug)]
#[clap(about, version=&VERSION.deref()[..], before_help=BANNER, after_help=HOMEPAGE, author)]
pub struct Args {
    #[clap(short, long, default_value = "config.toml")]
    pub config: PathBuf,
    #[clap(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand, PartialEq, Eq, Debug)]
pub enum SubCommand {
    #[clap(about = "Generate nginx.conf")]
    GenerateNginx(nginx::Command),
    #[clap(about = "Generate systemd.conf")]
    GenerateSystemd(systemd::Command),
    #[clap(about = "Generate a token")]
    GenerateToken(token::Command),
    #[clap(about = "Start a http Server")]
    Web(web::Command),
}

#[derive(clap::Parser, PartialEq, Eq, Debug)]
pub struct I18nSync {
    #[clap(short, long)]
    pub folder: String,
}

pub async fn launch() -> Result<()> {
    let args = Args::parse();
    if let SubCommand::GenerateNginx(ref it) = args.command {
        return nginx::launch(&it.domain, it.port);
    }
    if let SubCommand::GenerateSystemd(ref it) = args.command {
        return systemd::launch(&it.domain);
    }
    if let SubCommand::GenerateToken(ref it) = args.command {
        {
            log::info!("load config from {}", args.config.display());
            check_config_permission(&args.config)?;
        }
        let cfg: token::Config = from_toml(&args.config)?;
        return token::launch(&cfg, &it.user, it.years);
    }
    if let SubCommand::Web(ref it) = args.command {
        {
            log::info!("load config from {}", args.config.display());
            check_config_permission(&args.config)?;
        }
        let cfg: web::Config = from_toml(&args.config)?;
        return web::launch(&cfg, it.port, it.threads).await;
    }

    Ok(())
}
