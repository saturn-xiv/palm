pub mod author;
pub mod favicon;
pub mod keywords;

use icu::locale::Locale;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use super::super::{
    Result,
    cache::{FlexBuffersCacher, redis::StandaloneConnection as Cache},
    models::locale::{Dao as LocaleDao, I18n},
    orm::postgresql::Connection as Db,
};

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "SiteLayout")]
pub struct Layout {
    pub favicon: Option<String>,
    pub title: String,
    pub subhead: String,
    pub author: author::Author,
    pub keywords: Vec<String>,
    pub description: String,
    pub copyright: String,
    pub languages: Vec<String>,
}

impl Layout {
    pub const TITLE: &str = "site.title";
    pub const SUBHEAD: &str = "site.subhead";
    pub const DESCRIPTION: &str = "site.description";
    pub const COPYRIGHT: &str = "site.copyright";

    pub const KEY: &str = "site.layout";

    pub fn new(db: &mut Db, cache: &mut Cache, locale: &Locale) -> Result<Self> {
        if let Ok(it) = cache.get(Self::KEY) {
            return Ok(it);
        }
        let it = Self {
            favicon: favicon::Favicon::new(db).map(|x| x.0).ok(),
            title: I18n::t(db, locale, Self::TITLE, None::<&String>),
            subhead: I18n::t(db, locale, Self::SUBHEAD, None::<&String>),
            description: I18n::t(db, locale, Self::DESCRIPTION, None::<&String>),
            copyright: I18n::t(db, locale, Self::COPYRIGHT, None::<&String>),
            languages: LocaleDao::languages(db)?,
            author: author::Author::new(db).unwrap_or_default(),
            keywords: keywords::Keywords::new(db).unwrap_or_default().0,
        };
        cache.set(Self::KEY, &it, None)?;
        Ok(it)
    }
}
