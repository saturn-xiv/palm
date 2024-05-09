pub mod attachments;

use std::collections::HashMap;

use actix_web::web;
use hibiscus::{env::Thrift, Result};

use super::{models::locale::Dao as LocaleDao, orm::postgresql::Connection as Db};

pub type Jasmine = Thrift;

pub fn register(config: &mut web::ServiceConfig) {
    if cfg!(debug_assertions) {
        config
            .service(
                actix_files::Files::new("/3rd", "node_modules")
                    .prefer_utf8(true)
                    .use_etag(true)
                    .use_last_modified(true),
            )
            .service(
                actix_files::Files::new("/public", "assets")
                    .prefer_utf8(true)
                    .use_etag(true)
                    .use_last_modified(true),
            );
    }
    config.service(web::scope("/api").service(attachments::create));
}

pub fn i18n(db: &mut Db, lang: &str) -> Result<HashMap<String, String>> {
    let it = LocaleDao::by_lang(db, lang)?
        .iter()
        .map(|x| (x.code.clone(), x.message.clone()))
        .collect::<HashMap<_, _>>();
    Ok(it)
}
