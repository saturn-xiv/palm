use std::ops::DerefMut;
use std::str::FromStr;

use casbin::Enforcer;
use diesel::Connection as DieselConnection;
use juniper::{GraphQLInputObject, GraphQLObject};
use language_tags::LanguageTag;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, session::Session,
    themes::Layout, Error, Result,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::super::{models::locale::I18n, session::current_user};

#[derive(GraphQLObject)]
#[graphql(name = "GetSiteInfoByLangResponse")]
pub struct ByLang {
    pub title: String,
    pub subhead: String,
    pub description: String,
    pub copyright: String,
}

impl ByLang {
    pub fn new(db: &DbPool, lang: &str) -> Result<Self> {
        let lang = {
            let it = LanguageTag::from_str(lang)?;
            it.to_string()
        };
        let mut db = db.get()?;
        let db = db.deref_mut();

        Ok(Self {
            title: I18n::t(db, &lang, Layout::TITLE, None::<String>),
            subhead: I18n::t(db, &lang, Layout::SUBHEAD, None::<String>),
            description: I18n::t(db, &lang, Layout::DESCRIPTION, None::<String>),
            copyright: I18n::t(db, &lang, Layout::COPYRIGHT, None::<String>),
        })
    }
}

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "SetSiteInfoRequest")]
pub struct Base {
    #[validate(length(min = 1, max = 127))]
    pub title: String,
    #[validate(length(min = 2, max = 31))]
    pub subhead: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(length(min = 1, max = 63))]
    pub copyright: String,
}

impl Base {
    pub async fn save(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        lang: &str,
    ) -> Result<()> {
        let lang = {
            let it = LanguageTag::from_str(lang)?;
            it.to_string()
        };
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            user.is_administrator(enf)?;
        }

        db.transaction::<_, Error, _>(|db| {
            I18n::set(db, &lang, Layout::TITLE, &self.title)?;
            I18n::set(db, &lang, Layout::SUBHEAD, &self.subhead)?;
            I18n::set(db, &lang, Layout::DESCRIPTION, &self.description)?;
            I18n::set(db, &lang, Layout::COPYRIGHT, &self.copyright)?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Validate, Serialize, Deserialize)]
pub struct Keywords {
    #[validate(length(min = 1, max = 63))]
    pub items: Vec<String>,
}
