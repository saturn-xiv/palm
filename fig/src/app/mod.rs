pub mod cache;
pub mod generate;
pub mod i18n;
pub mod rpc;
pub mod user;
pub mod web;
pub mod worker;

use std::ops::Deref;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use hyper::StatusCode;
use log::info;
use palm::{is_stopped, HttpError, Result, BANNER, HOMEPAGE, VERSION};

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
    GenerateNginx(generate::Nginx),
    #[clap(about = "Generate systemd.conf")]
    GenerateSystemd(generate::Systemd),
    #[clap(about = "List all cache items")]
    CacheList,
    #[clap(about = "Clear cache items")]
    CacheClear,
    #[clap(about = "Create a new user")]
    UserCreate(user::Create),
    #[clap(about = "List all users")]
    UserList,
    #[clap(about = "Apply role to user(by uid)")]
    UserApplyRole(user::Role),
    #[clap(about = "Exempt role from user(by uid)")]
    UserExemptRole(user::Role),
    #[clap(about = "Reset user's password(by uid)")]
    UserResetPassword(user::ResetPassword),
    #[clap(about = "Generate user token(by uid)")]
    UserToken(user::Token),
    #[clap(about = "Sync i18n items")]
    I18nSync(i18n::Sync),
    #[clap(about = "Http Server")]
    Web(web::Server),
    #[clap(about = "gRPC Server")]
    Rpc(rpc::Server),
    #[clap(about = "Start a queue consumer")]
    Worker(worker::Consumer),
}

pub async fn launch() -> Result<()> {
    let args = Args::parse();

    if let SubCommand::GenerateNginx(ref it) = args.command {
        return it.launch();
    }
    if let SubCommand::GenerateSystemd(ref it) = args.command {
        return it.launch();
    }

    {
        info!("load config from {}", args.config.display());
        palm::check_config_permission(&args.config)?;
    }

    if let SubCommand::CacheClear = args.command {
        return cache::clear(args.config);
    }
    if let SubCommand::CacheList = args.command {
        return cache::list(args.config);
    }

    if let SubCommand::UserList = args.command {
        return user::list(args.config);
    }
    if let SubCommand::UserCreate(ref it) = args.command {
        return it.launch(args.config);
    }
    if let SubCommand::UserApplyRole(ref it) = args.command {
        return it.apply(args.config).await;
    }
    if let SubCommand::UserExemptRole(ref it) = args.command {
        return it.exempt(args.config).await;
    }
    if let SubCommand::UserToken(ref it) = args.command {
        return it.launch(args.config);
    }
    if let SubCommand::UserResetPassword(ref it) = args.command {
        return it.launch(args.config);
    }

    if let SubCommand::I18nSync(ref it) = args.command {
        return it.launch(args.config);
    }

    if is_stopped() {
        return Err(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some(".stop file exists!".to_string()),
        )));
    }
    if let SubCommand::Web(ref it) = args.command {
        return it.launch(args.config).await;
    }
    if let SubCommand::Rpc(ref it) = args.command {
        return it.launch(args.config).await;
    }
    if let SubCommand::Worker(ref it) = args.command {
        return match it.job {
            worker::Job::SendEmail => it.launch_send_email(args.config).await,
            worker::Job::SendSms => it.launch_send_sms(args.config).await,
        };
    }
    Ok(())
}
