pub mod locale;

use std::path::Path;

use hyacinth::schema::locales;
use portal::{
    Result,
    models::locale::Dao as LocaleDao,
    orm::postgresql::{Connection as Db, Node as PostgreSql},
};
use serde::{Deserialize, Serialize};

pub async fn seeds<P: AsRef<Path>>(_config: P, _locales: Option<Vec<String>>) -> Result<()> {
    // TODO
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    postgresql: PostgreSql,
}
