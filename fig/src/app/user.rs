use std::ops::DerefMut;
use std::path::Path;
use std::sync::Arc;

use casbin::{prelude::RbacApi, Enforcer};
use clap::Parser;
use daffodil::models::{
    log::{Dao as LogDao, Level as LogLevel},
    user::{email::Dao as EmailUserDao, Dao as UserDao, Item as User},
};
use diesel::Connection as DieselConntection;
use petunia::{
    hostname,
    orm::postgresql::{Config as PostgreSql, Pool as DbPool},
    parser::from_toml,
    queue::amqp::Config as RabbitMq,
    Error, Result,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct CreateByEmail {
    #[clap(short, long)]
    pub real_name: String,
    #[clap(short, long)]
    pub nickname: String,
    #[clap(short, long)]
    pub email: String,
    #[clap(short, long)]
    pub password: String,
}

impl CreateByEmail {
    pub fn launch<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let config: Config = from_toml(config_file)?;
        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let uid = Uuid::new_v4().to_string();
            let ip = hostname()?;
            let nickname = self.nickname.trim().to_lowercase();
            let email = self.email.trim().to_lowercase();
            let real_name = self.real_name.trim();
            db.transaction::<_, Error, _>(move |db| {
                UserDao::create(db, &uid, &User::guest_lang()?, User::guest_timezone())?;
                let iu = UserDao::by_uid(db, &uid)?;

                EmailUserDao::create(db, iu.id, real_name, &nickname, &email, &self.password)?;

                let eu = EmailUserDao::by_email(db, &email)?;
                EmailUserDao::confirm(db, eu.id)?;
                LogDao::create::<_, User>(
                    db,
                    iu.id,
                    super::NAME,
                    LogLevel::Info,
                    &ip,
                    None,
                    "create user by system user",
                )?;
                Ok(())
            })?;
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
        let (db, enforcer, user, role) = self.open(config_file).await?;
        {
            let mut it = enforcer.lock().await;
            it.add_role_for_user(&user, &role, None).await?;
        }
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let ip = hostname()?;

            db.transaction::<_, Error, _>(move |db| {
                let iu = UserDao::by_uid(db, &user)?;
                LogDao::create::<_, User>(
                    db,
                    iu.id,
                    super::NAME,
                    LogLevel::Info,
                    &ip,
                    None,
                    format!("apply role {} by system user", role),
                )?;
                Ok(())
            })?;
        }
        Ok(())
    }

    pub async fn exempt<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let (db, enforcer, user, role) = self.open(config_file).await?;

        {
            let mut it = enforcer.lock().await;
            it.delete_role_for_user(&user, &role, None).await?;
        }
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let ip = hostname()?;

            db.transaction::<_, Error, _>(move |db| {
                let iu = UserDao::by_uid(db, &user)?;
                LogDao::create::<_, User>(
                    db,
                    iu.id,
                    super::NAME,
                    LogLevel::Info,
                    &ip,
                    None,
                    format!("exempt role {} by system user", role),
                )?;
                Ok(())
            })?;
        }
        Ok(())
    }

    async fn open<P: AsRef<Path>>(
        &self,
        config_file: P,
    ) -> Result<(DbPool, Arc<Mutex<Enforcer>>, String, String)> {
        let config: Config = from_toml(config_file)?;
        let db = config.postgresql.open()?;
        let queue = Arc::new(config.rabbitmq.open());
        let enforcer = daffodil::rbac::new(db.clone(), queue).await?;
        let user = {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let it = UserDao::by_uid(db, &self.user)?;
            it.uid
        };
        let role = self.role.trim().to_lowercase();
        Ok((db, enforcer, user, role))
    }
}

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct ResetPasswordByEmail {
    #[clap(short, long)]
    pub email: String,
    #[clap(short, long)]
    pub password: String,
}

impl ResetPasswordByEmail {
    pub fn launch<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let config: Config = from_toml(config_file)?;
        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();
            let ip = hostname()?;
            let email = self.email.trim().to_lowercase();
            db.transaction::<_, Error, _>(move |db| {
                let eu = EmailUserDao::by_email(db, &email)?;
                EmailUserDao::set_password(db, eu.id, &self.password)?;
                LogDao::create::<_, User>(
                    db,
                    eu.user_id,
                    super::NAME,
                    LogLevel::Info,
                    &ip,
                    None,
                    "reset password by system user",
                )?;
                Ok(())
            })?;
        }
        Ok(())
    }
}

pub fn list<P: AsRef<Path>>(config_file: P) -> Result<()> {
    let config: Config = from_toml(config_file)?;
    let db = config.postgresql.open()?;
    {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let total = EmailUserDao::total(db)?;
        let page = 20;
        println!("{:<32} {:<38} EMAIL", "NICKNAME", "UID");
        for i in 1.. {
            for eu in EmailUserDao::index(db, (i - 1) * page, page)?.iter() {
                let iu = UserDao::by_id(db, eu.user_id)?;
                println!("{:<32} {:<38} {}", eu.nickname, iu.uid, eu.email);
            }
            if i * page >= total {
                break;
            }
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub postgresql: PostgreSql,
    pub rabbitmq: RabbitMq,
}
