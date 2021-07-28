pub mod graphql;
pub mod models;

use super::super::{orm::postgresql::Connection as Db, HttpError, Result};

use self::models::log::Dao as LogDao;

pub async fn fetch(db: &Db, url: &str) -> Result<()> {
    let res = reqwest::get(url).await?;
    let status = res.status();
    let body = res.text().await?;
    if !status.is_success() {
        return Err(Box::new(HttpError(status, Some(body))));
    }
    if let Ok(it) = LogDao::latest(db, url) {
        if it.body == body {
            debug!("didn't change, skip");
            return Ok(());
        }
    }
    LogDao::add(db, url, &body)?;
    Ok(())
}
