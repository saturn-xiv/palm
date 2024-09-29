pub mod i18n;
pub mod nginx;
pub mod systemd;
pub mod user;
pub mod web;

use std::ops::Deref;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use petunia::{Result, BANNER, HOMEPAGE, VERSION};

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
    #[clap(about = "Create a new user by email")]
    UserCreateByEmail(user::CreateByEmail),
    #[clap(about = "List all users")]
    UserList,
    #[clap(about = "Apply role to user(by uid)")]
    UserApplyRole(user::Role),
    #[clap(about = "Exempt role from user(by uid)")]
    UserExemptRole(user::Role),
    #[clap(about = "Reset user's password(by email)")]
    UserResetPasswordByEmail(user::ResetPasswordByEmail),
    #[clap(about = "Sync i18n records from filesystem")]
    I18nSync(i18n::Command),
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
        return it.launch();
    }
    if let SubCommand::GenerateSystemd(ref it) = args.command {
        return it.launch();
    }
    if let SubCommand::UserCreateByEmail(ref it) = args.command {
        return it.launch(&args.config);
    }
    if let SubCommand::UserResetPasswordByEmail(ref it) = args.command {
        return it.launch(&args.config);
    }
    if let SubCommand::UserApplyRole(ref it) = args.command {
        return it.apply(&args.config).await;
    }
    if let SubCommand::UserExemptRole(ref it) = args.command {
        return it.exempt(&args.config).await;
    }
    Ok(())
}
