use std::ops::DerefMut;
use std::path::Path;
use std::sync::Arc;

use camelia::{
    models::{
        log::{Dao as LogDao, Level as LogLevel},
        user::{Action, Dao as UserDao, Item as User},
    },
    orm::postgresql::Config as PostgreSql,
};
use casbin::RbacApi;
use chrono::Duration;
use clap::Parser;
use diesel::Connection as DieselConntection;
use hyper::StatusCode;
use log::{info, warn};
use palm::{
    crypto::{hmac::Hmac, Key},
    jwt::{openssl::Jwt, Jwt as JwtProvider},
    parser::from_toml,
    queue::rabbitmq::{amqp::Amqp, Config as RabbitMq},
    rbac::v1 as rbac_v1,
    Error, HttpError, Result,
};
use serde::{Deserialize, Serialize};

use super::super::NAME;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TokenConfig {
    #[serde(rename = "secret-key")]
    secret_key: Key,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CreateConfig {
    #[serde(rename = "secret-key")]
    pub secret_key: Key,
    postgresql: PostgreSql,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RoleConfig {
    #[serde(rename = "secret-key")]
    pub secret_key: Key,
    postgresql: PostgreSql,
    rabbitmq: RabbitMq,
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
        let mac = Hmac::new(&config.secret_key.0)?;
        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            if UserDao::by_nickname(db, &self.nickname).is_ok() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some(format!("user {} already existed", self.nickname)),
                )));
            }
            if UserDao::by_email(db, &self.email).is_ok() {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some(format!("user {} already existed", self.email)),
                )));
            }
            let user = db.transaction::<_, Error, _>(move |db| {
                UserDao::sign_up(
                    db,
                    &mac,
                    "Nil Gate",
                    &self.nickname,
                    &self.email,
                    &self.password,
                    &"en-US".parse()?,
                    &"UTC".parse()?,
                )?;
                let user = UserDao::by_email(db, &self.email)?;
                UserDao::confirm(db, user.id)?;
                LogDao::add::<String, User>(
                    db,
                    user.id,
                    NAME,
                    LogLevel::Info,
                    &hostname(),
                    Some(user.id),
                    format!("Created by system user {}.", current_user()),
                )?;
                Ok(user)
            })?;
            info!("create user {}", user);
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
        let jwt = Jwt::new(&config.secret_key.0);

        {
            let token = jwt.sign(
                NAME,
                &self.subject,
                &Action::SignIn.to_string(),
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
    fn role(&self) -> String {
        let it = match &self.role[..] {
            "admin" => rbac_v1::Role::administrator(),
            "root" => rbac_v1::Role::root(),
            other => rbac_v1::Role::member(other.to_string()),
        };
        it.to_string()
    }
    pub async fn apply<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let role = self.role();
        let config: RoleConfig = from_toml(config_file)?;
        let rabbitmq = Arc::new(config.rabbitmq.open().await?);
        let enforcer = config.postgresql.casbin_enforcer(rabbitmq).await?;

        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let user = UserDao::by_nickname(db, &self.user)?;

            {
                let user = user.to_subject();
                let mut enforcer = enforcer.lock().await;

                if enforcer.has_role_for_user(&user, &role, None) {
                    warn!("user({}) already has role({})", self.user, self.role);
                    return Ok(());
                }
                enforcer.add_role_for_user(&user, &role, None).await?;
            }
            LogDao::add::<String, User>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &hostname(),
                Some(user.id),
                format!("Apply role {} by system user {}", self.role, current_user()),
            )?;
            info!("apple role {} to user {}", self.role, user);
        }

        Ok(())
    }
    pub async fn exempt<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let role = self.role();
        let config: RoleConfig = from_toml(config_file)?;
        let rabbitmq = Arc::new(config.rabbitmq.open().await?);
        let enforcer = config.postgresql.casbin_enforcer(rabbitmq).await?;

        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let user = UserDao::by_nickname(db, &self.user)?;

            {
                let user = user.to_subject();
                let mut enforcer = enforcer.lock().await;
                if !enforcer.has_role_for_user(&user, &role, None) {
                    warn!("user({}) didn't has role({})", self.user, self.role);
                    return Ok(());
                }
                enforcer.delete_role_for_user(&user, &role, None).await?;
            }

            LogDao::add::<String, User>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &hostname(),
                Some(user.id),
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
        let mac = Hmac::new(&config.secret_key.0)?;
        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let user = UserDao::by_nickname(db, &self.user)?;
            db.transaction::<_, Error, _>(move |db| {
                UserDao::password(db, &mac, user.id, &self.password)?;
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

        let total = UserDao::count(db)?;
        let page = 20;
        println!("{:<6} {:<32} USER", "ID", "NICKNAME");
        for i in 1.. {
            for it in UserDao::all(db, (i - 1) * page, page)?.iter() {
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
