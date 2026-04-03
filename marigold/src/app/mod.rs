pub mod db;
pub mod http;
pub mod rpc;
pub mod user;
pub mod workers;

use std::{path::PathBuf, time::Duration};

use clap::{Parser, Subcommand};
use phlox::Result;

use super::{BANNER, DESCRIPTION, GIT_VERSION, HOMEPAGE, NAME};

#[derive(Debug, Parser)]
#[command(name = NAME, version = GIT_VERSION, about = DESCRIPTION, before_help = BANNER, after_help = HOMEPAGE, long_about = None, propagate_version = true, arg_required_else_help = true)]
struct Cli {
    #[arg(
        short,
        long,
        help = "Load configuration from file(toml)",
        default_value = "config.toml"
    )]
    config: PathBuf,
    #[arg(short, long, help = "Run on debug mode")]
    debug: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        arg_required_else_help = true,
        about = "Load data into database, init queues"
    )]
    DbSeeds {
        #[arg(short, long, help = "Load locales from(yaml)")]
        locales: Option<Vec<String>>,
    },
    #[command(arg_required_else_help = true, about = "List all users")]
    ListUser,
    #[command(arg_required_else_help = true, about = "Create an email user")]
    CreateUserByEmail {
        #[arg(short, long, required = true, help = "Username")]
        name: String,
        #[arg(short, long, required = true, help = "Email")]
        email: String,
        #[arg(short, long, required = true, help = "Password")]
        password: String,
    },
    #[command(arg_required_else_help = true, about = "Add role for user")]
    AddRoleForUser {
        #[arg(
            short,
            long,
            required = true,
            help = "Role's name(root, administrator, etc.)"
        )]
        role: String,
        #[arg(short, long, required = true, help = "User's SN")]
        user: String,
    },
    #[command(arg_required_else_help = true, about = "Delete role for user")]
    DeleteRoleForUser {
        #[arg(
            short,
            long,
            required = true,
            help = "Role's name(root, administrator, etc.)"
        )]
        role: String,
        #[arg(short, long, required = true, help = "User's SN")]
        user: String,
    },
    #[command(arg_required_else_help = true, about = "Start an email-send worker")]
    EmailSendWorker {
        #[arg(
            short,
            long,
            required = true,
            help = "Interval by microseconds",
            default_value_t = 5_000
        )]
        interval: u64,
        #[arg(short, long, required = true, help = "Queue name")]
        queue: String,
    },
    #[command(arg_required_else_help = true, about = "Start a sms-send worker")]
    SmsSendWorker {
        #[arg(
            short,
            long,
            required = true,
            help = "Interval by microseconds",
            default_value_t = 5_000
        )]
        interval: u64,
        #[arg(short, long, required = true, help = "Queue name")]
        queue: String,
    },
    #[command(arg_required_else_help = true, about = "Start a cups worker")]
    CupsWorker {
        #[arg(
            short,
            long,
            required = true,
            help = "Interval by microseconds",
            default_value_t = 5_000
        )]
        interval: u64,
        #[arg(short, long, required = true, help = "Queue name")]
        queue: String,
    },
    #[command(arg_required_else_help = true, about = "Start a TeX worker")]
    TexWorker {
        #[arg(
            short,
            long,
            required = true,
            help = "Interval by microseconds",
            default_value_t = 5_000
        )]
        interval: u64,
        #[arg(short, long, required = true, help = "Queue name")]
        queue: String,
    },
    #[command(arg_required_else_help = true, about = "Start a gRPC server")]
    Rpc {
        #[arg(short, long, required = true, help = "Port", default_value_t = 8080)]
        port: u16,
    },
    #[command(arg_required_else_help = true, about = "Start a HTTP server")]
    Http {
        #[arg(short, long, required = true, help = "Port", default_value_t = 8080)]
        port: u16,
        #[arg(
            short,
            long,
            required = true,
            help = "Theme",
            default_value_t = http::Theme::Bootstrap,
        )]
        theme: http::Theme,
    },
}

pub async fn run() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::DbSeeds { locales } => db::seeds(&args.config, locales).await,
        Commands::ListUser => user::list(&args.config),
        Commands::CreateUserByEmail {
            ref name,
            ref email,
            ref password,
        } => user::create_by_email(&args.config, name, email, password),
        Commands::AddRoleForUser { ref role, ref user } => {
            user::add_role(&args.config, user, role).await
        }
        Commands::DeleteRoleForUser { ref role, ref user } => {
            user::delete_role(&args.config, user, role).await
        }
        Commands::EmailSendWorker {
            interval,
            ref queue,
        } => workers::email_send::start(&args.config, queue, Duration::from_micros(interval)).await,
        Commands::SmsSendWorker {
            interval,
            ref queue,
        } => workers::sms_send::start(&args.config, queue, Duration::from_micros(interval)).await,
        Commands::CupsWorker {
            interval,
            ref queue,
        } => workers::cups::start(&args.config, queue, Duration::from_micros(interval)).await,
        Commands::TexWorker {
            interval,
            ref queue,
        } => workers::tex::start(&args.config, queue, Duration::from_micros(interval)).await,
        Commands::Http { port, theme } => http::start(&args.config, port, theme).await,
        Commands::Rpc { port } => rpc::start(&args.config, port).await,
    }
}
