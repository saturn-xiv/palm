use std::collections::HashSet;

use casbin::Enforcer;
use chrono::{NaiveDateTime, Utc};
use juniper::{GraphQLInputObject, GraphQLObject};
use palm::{
    cache::redis::ClusterConnection as Cache, crypto::aes::Aes, jwt::Jwt, session::Session, Error,
    GIT_VERSION,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::super::{
    i18n::I18n,
    models::{locale::Dao as LocaleDao, setting::FlatBuffer},
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "SiteAuthorRequestItem")]
pub struct AuthorRequest {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[validate(email, length(min = 1, max = 63))]
    pub email: String,
}

#[derive(Validate, GraphQLObject, Serialize, Deserialize, Default)]
#[graphql(name = "SiteAuthor")]
pub struct Author {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[validate(email, length(min = 1, max = 63))]
    pub email: String,
}
#[derive(GraphQLObject)]
#[graphql(name = "SiteInfoResponse")]
pub struct Response {
    pub locale: String,
    pub title: String,
    pub subhead: String,
    pub description: String,
    pub copyright: String,
    pub favicon: String,
    pub keywords: Vec<String>,
    pub authors: Vec<Author>,
    pub languages: Vec<String>,
    pub icp_code: Option<IcpCode>,
    pub gab_code: Option<GabCode>,
    pub created_at: NaiveDateTime,
    pub version: String,
}

impl Response {
    pub fn new(db: &mut Db, _ch: &mut Cache, aes: &Aes, lang: &str) -> Result<Self, Error> {
        // TODO cache
        let it = Self {
            locale: lang.to_string(),
            title: I18n::t(db, lang, InfoRequest::TITLE, &None::<String>),
            subhead: I18n::t(db, lang, InfoRequest::SUBHEAD, &None::<String>),
            description: I18n::t(db, lang, InfoRequest::DESCRIPTION, &None::<String>),
            copyright: I18n::t(db, lang, InfoRequest::COPYRIGHT, &None::<String>),
            keywords: {
                let items = FlatBuffer::get::<Keywords, _>(db, aes, None)
                    .map(|x| x.items)
                    .unwrap_or_default();
                Vec::from_iter(items)
            },
            icp_code: FlatBuffer::get::<IcpCode, _>(db, aes, None).ok(),
            gab_code: FlatBuffer::get::<GabCode, _>(db, aes, None).ok(),
            authors: FlatBuffer::get::<Authors, _>(db, aes, None)
                .map(|x| x.items)
                .unwrap_or_default(),
            favicon: FlatBuffer::get::<Favicon, _>(db, aes, None)
                .map(|x| x.url)
                .unwrap_or("/my/palm.svg".to_string()),
            languages: LocaleDao::languages(db)?,
            version: GIT_VERSION.to_string(),
            created_at: Utc::now().naive_utc(),
        };

        Ok(it)
    }
}

#[derive(Validate, GraphQLObject, Serialize, Deserialize, Default)]
#[graphql(name = "WebSiteGabCode")]
pub struct GabCode {
    #[validate(length(min = 1, max = 63))]
    pub code: String,
    #[validate(length(min = 1, max = 63))]
    pub name: String,
}

#[derive(Validate, GraphQLObject, Serialize, Deserialize, Default)]
#[graphql(name = "WebSiteIcpCode")]
pub struct IcpCode {
    #[validate(length(min = 1, max = 63))]
    pub code: String,
}

#[derive(Validate, Serialize, Deserialize, Default)]
pub struct Keywords {
    pub items: HashSet<String>,
}

impl Keywords {
    pub fn new(items: &[String]) -> Self {
        let items: Vec<String> = items.iter().map(|x| x.trim().to_lowercase()).collect();
        Self {
            items: HashSet::from_iter(items),
        }
    }
}

#[derive(Validate, Serialize, Deserialize, Default)]
pub struct Authors {
    pub items: Vec<Author>,
}

#[derive(Validate, Serialize, Deserialize, Default)]
pub struct Favicon {
    #[validate(length(min = 1, max = 127))]
    pub url: String,
}

#[derive(Validate)]
pub struct InfoRequest {
    #[validate(length(min = 1, max = 63))]
    pub title: String,
    #[validate(length(min = 1, max = 31))]
    pub subhead: String,
    #[validate(length(min = 1, max = 511))]
    pub description: String,
    #[validate(length(min = 1, max = 31))]
    pub copyright: String,
}
impl InfoRequest {
    pub const TITLE: &'static str = "site.title";
    pub const SUBHEAD: &'static str = "site.subhead";
    pub const DESCRIPTION: &'static str = "site.description";
    pub const COPYRIGHT: &'static str = "site.copyright";

    pub async fn handle<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
    ) -> Result<(), Error> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        user.is_administrator(enf).await?;

        I18n::set(db, &ss.lang, Self::TITLE, &self.title)?;
        I18n::set(db, &ss.lang, Self::SUBHEAD, &self.subhead)?;
        I18n::set(db, &ss.lang, Self::DESCRIPTION, &self.description)?;
        I18n::set(db, &ss.lang, Self::COPYRIGHT, &self.copyright)?;
        Ok(())
    }
}
