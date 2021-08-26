pub mod models;
pub mod services;
pub mod v1;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use askama::Template;
use clap::Clap;

use super::super::{orm::postgresql::Connection as Db, HttpError, Result, NAME};

use self::models::{
    log::Dao as LogDao,
    site::{Dao as SiteDao, Item as Site},
};

#[derive(Clap)]
pub enum Crawler {
    #[clap(about = "Fetch from a url")]
    Fetch(Fetch),
    #[clap(about = "Export a crontab script file")]
    Crontab,
}

#[derive(Clap)]
pub struct Fetch {
    #[clap(short, long)]
    pub url: String,
}

impl Fetch {
    pub async fn fetch(&self, db: &Db) -> Result<()> {
        let site = SiteDao::by_url(db, &self.url)?;
        let res = reqwest::get(&self.url).await?;
        let status = res.status();
        let body = res.text().await?;
        if !status.is_success() {
            return Err(Box::new(HttpError(status, Some(body))));
        }
        if let Ok(it) = LogDao::latest(db, site.id) {
            if it.body == body {
                debug!("didn't change, skip");
                return Ok(());
            }
        }
        LogDao::add(db, site.id, &body)?;
        Ok(())
    }
}

pub fn export<P: AsRef<Path>>(cfg: P, db: &Db) -> Result<()> {
    let cfg = cfg.as_ref();
    let it = Crontab {
        config: cfg.display().to_string(),
        name: NAME.to_string(),
        items: SiteDao::all(db)?,
    };
    let body = it.render()?;
    let file = Path::new("tmp").join("crontab");
    info!("generate file {}", file.display());
    let mut file = File::create(&file)?;
    file.write_all(body.as_bytes())?;
    info!("please copy it into /etc/cron.d folder.");
    Ok(())
}

#[derive(Template)]
#[template(path = "crawler/crontab", escape = "none")]
pub struct Crontab {
    pub config: String,
    pub name: String,
    pub items: Vec<Site>,
}
