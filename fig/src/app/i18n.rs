use std::ops::DerefMut;
use std::path::Path;

use camelia::{models::locale::Dao as LocaleDao, orm::postgresql::Config as PostgreSql};
use clap::Parser;
use diesel::Connection as DieselConntection;
use palm::{parser::from_toml, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Parser, PartialEq, Eq, Debug)]
pub struct Sync {
    #[clap(short, long)]
    pub folder: String,
}

impl Sync {
    pub fn launch<P: AsRef<Path>>(&self, config_file: P) -> Result<()> {
        let config: Config = from_toml(config_file)?;
        let db = config.postgresql.open()?;
        {
            let mut db = db.get()?;
            let db = db.deref_mut();

            db.transaction::<_, Error, _>(move |db| {
                LocaleDao::sync(db, &self.folder)?;
                Ok(())
            })?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub postgresql: PostgreSql,
}
