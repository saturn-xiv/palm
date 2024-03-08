use std::ops::DerefMut;
use std::path::Path;

use camelia::{models::user::Dao as UserDao, orm::postgresql::Config as PostgreSql};
use clap::Parser;
use palm::{parser::from_toml, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub postgresql: PostgreSql,
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
    pub fn launch<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Token {
    #[clap(short, long)]
    pub user: String,
    #[clap(short, long, default_value_t = 1<<12)]
    pub weeks: u32,
}

impl Token {
    pub fn launch<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Role {
    #[clap(short, long)]
    pub user: String,
    #[clap(short, long)]
    pub role: String,
}

impl Role {
    pub fn apply<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
    pub fn exempt<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
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
    pub fn launch<P: AsRef<Path>>(&self, _config_file: P) -> Result<()> {
        // TODO
        Ok(())
    }
}

pub fn list<P: AsRef<Path>>(config_file: P) -> Result<()> {
    let config: Config = from_toml(config_file)?;
    {
        let db = config.postgresql.open()?;
        let mut db = db.get()?;
        let db = db.deref_mut();

        let total = UserDao::count(db)?;
        let page = 20;
        println!("{:<6} ID", "USER");
        for i in 1.. {
            for it in UserDao::all(db, (i - 1) * page, page)?.iter() {
                println!("{:<6} {}", it.id, it);
            }
            if i * page >= total {
                break;
            }
        }
    }
    Ok(())
}
