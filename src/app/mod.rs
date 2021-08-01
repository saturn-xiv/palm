pub mod generate;
pub mod web;

use std::ops::Deref;
use std::path::PathBuf;

use clap::Clap;

use super::{
    i18n,
    orm::postgresql::console::Database,
    parser::from_toml,
    plugins::{crawler, nut, sms},
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
pub struct Opts {
    #[clap(short, long, about = "Config file", default_value = "config.toml")]
    pub config: PathBuf,
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(about = "Generate")]
    Generate(generate::Generate),
    #[clap(about = "PostgreSql")]
    Db(Database),
    #[clap(about = "Http Server")]
    Web,
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
                    sms::models::MIGRATION.deref(),
                ],
            )?;
        }
        SubCommand::Web => {
            let cfg: Config = from_toml(&opts.config)?;
            web::launch(&cfg).await?;
        }
    };
    Ok(())
}
