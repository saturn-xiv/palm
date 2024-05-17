use std::ops::DerefMut;
use std::path::Path;

use camelia::{
    models::{
        log::Dao as LogDao,
        user::{
            email::{Dao as EmailUserDao, Item as EmailUser},
            Item as User,
        },
    },
    orm::postgresql::Config as PostgreSql,
};
use chrono::Duration;
use clap::Parser;
use diesel::Connection as DieselConntection;
use hibiscus::parser::from_toml;
use hyper::StatusCode;
use log::info;
use palm::{
    camelia::v1::user_logs_response::item::Level as LogLevel, gourd::Policy, Error, HttpError, Jwt,
    Result, Thrift,
};
use serde::{Deserialize, Serialize};

use super::super::NAME;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TokenConfig {
    loquat: Thrift,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CreateConfig {
    loquat: Thrift,
    postgresql: PostgreSql,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RoleConfig {
    gourd: Thrift,
    postgresql: PostgreSql,
}

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
    pub fn launch<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let config: CreateConfig = from_toml(config_file)?;
        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            if EmailUserDao::by_nickname(db, &self.nickname).is_ok() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some(format!("user {} already existed", self.nickname)),
                )));
            }
            if EmailUserDao::by_email(db, &self.email).is_ok() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some(format!("user {} already existed", self.email)),
                )));
            }
            db.transaction::<_, Error, _>(move |db| {
                let user = EmailUserDao::create(
                    db,
                    &config.loquat,
                    EmailUser::GUEST_NAME,
                    &self.nickname,
                    &self.email,
                    &self.password,
                    (
                        &EmailUser::GUEST_LANG.parse()?,
                        EmailUser::GUEST_TIMEZONE.parse()?,
                    ),
                )?;
                {
                    let it = EmailUserDao::by_email(db, &self.email)?;
                    EmailUserDao::confirm(db, it.id)?;
                }
                LogDao::add::<String, User>(
                    db,
                    user.id,
                    NAME,
                    LogLevel::Info,
                    &hostname(),
                    Some(user.id),
                    format!("Created by system user {}.", current_user()),
                )?;
                Ok(())
            })?;
            info!("create user {}<{}>", self.nickname, self.email);
        }

        Ok(())
    }
}

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Token {
    #[clap(short, long, help = "UID")]
    pub user: String,
    #[clap(short, long)]
    pub issuer: String,
    #[clap(short, long)]
    pub subject: String,
    #[clap(short, long)]
    pub audience: String,
    #[clap(short, long, default_value_t = 1<<12)]
    pub weeks: u32,
}

impl Token {
    pub fn launch<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let config: TokenConfig = from_toml(config_file)?;

        {
            let token = Jwt::sign_by_duration(
                &config.loquat,
                &self.issuer,
                &self.subject,
                &self.audience,
                &None::<String>,
                Duration::try_weeks(self.weeks as i64).ok_or(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("bad weeks for jwt".to_string()),
                )))?,
            )?;
            println!("{token}");
        }
        Ok(())
    }
}

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Role {
    #[clap(short, long, help = "Nickname")]
    pub user: String,
    #[clap(short, long)]
    pub role: String,
}

impl Role {
    pub async fn apply<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let config: RoleConfig = from_toml(config_file)?;

        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let user = EmailUserDao::by_nickname(db, &self.user)?;
            Policy::add_roles_for_user(&config.gourd, user.user_id, &[&self.role])?;
            LogDao::add::<String, User>(
                db,
                user.user_id,
                NAME,
                LogLevel::Info,
                &hostname(),
                None,
                format!("Apply role {} by system user {}", self.role, current_user()),
            )?;
            info!("apple role {} to user {}", self.role, user);
        }
        Ok(())
    }
    pub async fn exempt<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let config: RoleConfig = from_toml(config_file)?;

        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let user = EmailUserDao::by_nickname(db, &self.user)?;

            Policy::delete_roles_for_user(&config.gourd, user.user_id, &[&self.role])?;

            LogDao::add::<String, User>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &hostname(),
                None,
                format!(
                    "Exempt role {} by system user {}",
                    self.role,
                    current_user()
                ),
            )?;
            info!("exempt role {} to user {}", self.role, user);
        }
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
    pub fn launch<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let config: CreateConfig = from_toml(config_file)?;

        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let user = EmailUserDao::by_nickname(db, &self.user)?;
            db.transaction::<_, Error, _>(move |db| {
                EmailUserDao::password(db, &config.loquat, user.id, &self.password)?;
                LogDao::add::<String, User>(
                    db,
                    user.id,
                    NAME,
                    LogLevel::Info,
                    &hostname(),
                    Some(user.id),
                    format!("Reset password by system user {}.", current_user()),
                )?;
                Ok(())
            })?;
            info!("reset password of user {}", user);
        }

        Ok(())
    }
}

pub fn list<P: AsRef<Path>>(config_file: P) -> Result<()> {
    let config: CreateConfig = from_toml(config_file)?;
    let db = config.postgresql.open()?;
    {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let total = EmailUserDao::count(db)?;
        let page = 20;
        println!("{:<6} {:<32} USER", "ID", "NICKNAME");
        for i in 1.. {
            for it in EmailUserDao::all(db, (i - 1) * page, page)?.iter() {
                println!("{:<6} {:<32} {}", it.id, it.nickname, it);
            }
            if i * page >= total {
                break;
            }
        }
    }
    Ok(())
}

fn hostname() -> String {
    if let Ok(ref it) = nix::sys::utsname::uname() {
        if let Some(it) = it.nodename().to_str() {
            return it.to_string();
        }
    }
    "unknown".to_string()
}

fn current_user() -> String {
    nix::unistd::User::from_uid(nix::unistd::getuid())
        .map_or_else(|_| None, |x| x.map(|y| y.name))
        .unwrap_or("unknown".to_string())
}
