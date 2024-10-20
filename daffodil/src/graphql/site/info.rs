use std::str::FromStr;
use std::{any::type_name, ops::DerefMut};

use casbin::Enforcer;
use diesel::Connection as DieselConnection;
use language_tags::LanguageTag;
use petunia::{
    crypto::Key,
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    themes::{Author as SiteAuthor, Layout},
    Error, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::super::{models::locale::I18n, session::current_user};
use super::{get, set};

#[derive(Validate)]
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

pub struct Author;

impl Author {
    pub fn get(db: &DbPool, secrets: Key, lang: &str) -> Result<SiteAuthor> {
        let it = get(db, secrets, Self::key(lang), None)?;
        Ok(it)
    }
    pub async fn save(
        ss: &Session,
        db: &DbPool,
        secrets: Key,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        lang: &str,
        value: &SiteAuthor,
    ) -> Result<()> {
        let lang = {
            let it = LanguageTag::from_str(lang)?;
            it.to_string()
        };
        value.validate()?;
        set(
            ss,
            db,
            secrets,
            jwt,
            enforcer,
            (Self::key(&lang), None, value, false),
        )
        .await?;

        Ok(())
    }

    pub fn key(lang: &str) -> String {
        format!("{}://{}", lang, type_name::<SiteAuthor>())
    }
}

pub struct Keywords;

impl Keywords {
    pub fn get(db: &DbPool, secrets: Key) -> Result<Vec<String>> {
        let it = get(db, secrets, Layout::KEYWORDS.to_string(), None)?;
        Ok(it)
    }
    pub async fn save(
        ss: &Session,
        db: &DbPool,
        secrets: Key,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        values: &Vec<String>,
    ) -> Result<()> {
        set(
            ss,
            db,
            secrets,
            jwt,
            enforcer,
            (Layout::KEYWORDS.to_string(), None, values, false),
        )
        .await?;

        Ok(())
    }
}
