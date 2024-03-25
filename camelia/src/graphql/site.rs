use chrono::{NaiveDateTime, Utc};
use juniper::GraphQLObject;
use palm::{
    cache::redis::ClusterConnection as Cache,
    crypto::{aes::Aes, Secret},
    Result,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::super::{
    i18n::I18n,
    models::{locale::Dao as LocaleDao, setting::Dao as SettingDao},
    orm::postgresql::Connection as Db,
};

#[derive(GraphQLObject, Serialize, Deserialize)]
#[graphql(name = "SiteAuthor")]
pub struct Author {
    pub name: String,
    pub email: String,
}
#[derive(GraphQLObject)]
#[graphql(name = "SiteInfoResponse")]
pub struct InfoResponse {
    pub title: String,
    pub subhead: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub authors: Vec<Author>,
    pub languages: Vec<String>,
    pub icp_code: Option<String>,
    pub gab_code: Option<GabCode>,
    pub created_at: NaiveDateTime,
}

#[derive(GraphQLObject, Serialize, Deserialize, Default)]
#[graphql(name = "WebSiteGabCode")]
pub struct GabCode {
    pub code: String,
    pub name: String,
}

impl InfoResponse {
    pub const TITLE: &'static str = "site.title";
    pub const SUBHEAD: &'static str = "site.subhead";
    pub const DESCRIPTION: &'static str = "site.description";
    pub const KEYWORDS: &'static str = "site.keywords";
    pub const AUTHORS: &'static str = "site.authors";
    pub const ICP_CODE: &'static str = "site.icp-code";
    pub const GAB_CODE: &'static str = "site.gab-code";

    fn get<V: Default + DeserializeOwned, S: Secret>(db: &mut Db, aes: &S, key: &str) -> Result<V> {
        let buf = SettingDao::get(db, aes, &key.to_string(), None)?;
        let it = flexbuffers::from_slice(&buf)?;
        Ok(it)
    }
    pub fn new(db: &mut Db, _ch: &mut Cache, aes: &Aes, lang: &str) -> Result<Self> {
        // TODO cache
        Ok(Self {
            title: I18n::t(db, lang, Self::TITLE, &None::<String>),
            subhead: I18n::t(db, lang, Self::SUBHEAD, &None::<String>),
            description: I18n::t(db, lang, Self::DESCRIPTION, &None::<String>),
            keywords: Self::get(db, aes, Self::KEYWORDS).unwrap_or_default(),
            icp_code: Self::get(db, aes, Self::ICP_CODE).ok(),
            gab_code: Self::get(db, aes, Self::GAB_CODE).ok(),
            authors: Self::get(db, aes, Self::AUTHORS).unwrap_or_default(),
            languages: LocaleDao::languages(db)?,
            created_at: Utc::now().naive_utc(),
        })
    }
}
