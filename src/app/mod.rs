pub mod generate;
pub mod web;

use std::ops::Deref;
use std::path::PathBuf;

use clap::{AppSettings, Clap};

use super::{
    i18n,
    orm::postgresql::console::Database,
    parser::from_toml,
    plugins::{crawler, nut, twilio},
    settings, Result, VERSION,
};

use super::env::Config;

#[derive(Clap)]
#[clap(
    about = env!("CARGO_PKG_DESCRIPTION"), 
    author = env!("CARGO_PKG_AUTHORS"),
    after_help = env!("CARGO_PKG_HOMEPAGE"),
    version = VERSION
)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long, about = "Config file", default_value = "config.toml")]
    pub config: PathBuf,
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(subcommand, about = "Generate")]
    Generate(generate::Generate),
    #[clap(subcommand, about = "PostgreSql")]
    Db(Database),
    #[clap(subcommand, about = "Crawler")]
    Crawler(crawler::Crawler),
    #[clap(about = "Http Server")]
    Web,
    #[clap(about = "Rpc Server")]
    Rpc,
    #[clap(about = "Worker process")]
    Worker,
}

pub async fn launch() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.sub_cmd {
        SubCommand::Generate(it) => {
            it.launch(&opts.config)?;
        }
        SubCommand::Db(opt) => {
            let cfg: Config = from_toml(&opts.config)?;
            let db = cfg.postgresql.open()?;
            let db = db.get()?;
            let db = db.deref();
            opt.launch(
                db,
                &[
                    settings::MIGRATION.deref(),
                    i18n::locale::MIGRATION.deref(),
                    nut::models::MIGRATION.deref(),
                    crawler::models::MIGRATION.deref(),
                    twilio::models::MIGRATION.deref(),
                ],
            )?;
        }
        SubCommand::Crawler(opt) => {
            let cfg: Config = from_toml(&opts.config)?;
            let db = cfg.postgresql.open()?;
            let db = db.get()?;
            let db = db.deref();
            match opt {
                crawler::Crawler::Crontab => {
                    crawler::export(&opts.config, db)?;
                }
                crawler::Crawler::Fetch(opt) => {
                    opt.fetch(db).await?;
                }
            }
        }
        SubCommand::Web => {
            let cfg: Config = from_toml(&opts.config)?;
            web::launch(&cfg).await?;
        }
        SubCommand::Rpc => {}
        SubCommand::Worker => {}
    };
    Ok(())
}
