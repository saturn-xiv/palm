use std::ops::DerefMut;
use std::str::FromStr;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::session::current_user;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};
use language_tags::LanguageTag;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, session::Session, Error,
    HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::forum::{Dao as ForumDao, Item as Forum};
use super::ROLE_MANAGER;

#[derive(GraphQLObject)]
#[graphql(name = "BbsForum")]
pub struct Item {
    pub id: i32,
    pub lang: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Forum> for Item {
    fn from(it: Forum) -> Self {
        Self {
            id: it.id,
            lang: it.lang.clone(),
            slug: it.slug.clone(),
            title: it.title.clone(),
            description: it.description.clone(),
            status: it.status.clone(),
            locked_at: it.locked_at,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        }
    }
}

impl Item {
    pub fn by_lang(db: &DbPool, lang: &str) -> Result<Vec<Self>> {
        let lang = {
            let it = LanguageTag::from_str(lang)?;
            it.to_string()
        };
        let mut db = db.get()?;
        let db = db.deref_mut();

        let mut items = Vec::new();
        for it in ForumDao::by_lang(db, &lang)? {
            items.push(it.into());
        }
        Ok(items)
    }
    pub fn by_id(db: &DbPool, id: i32) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let it = ForumDao::by_id(db, id)?;
        Ok(it.into())
    }
}

#[derive(Validate, GraphQLInputObject, Clone)]
#[graphql(name = "BbsForumCreateForm")]
pub struct Create {
    #[validate(length(min = 1, max = 127))]
    pub slug: String,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1, max = 511))]
    pub description: String,
}

impl Create {
    pub async fn execute(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enf: &Mutex<Enforcer>,
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
            let mut enf = enf.lock().await;
            let enf = enf.deref_mut();
            user.has(enf, ROLE_MANAGER)?;
        }

        db.transaction::<_, Error, _>(|db| {
            ForumDao::create(db, &lang, &self.slug, &self.title, &self.description)?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Validate)]
pub struct Update {
    #[validate(length(min = 1, max = 127))]
    pub slug: String,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1, max = 511))]
    pub description: String,
}

impl Update {
    pub async fn execute(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enf: &Mutex<Enforcer>,
        id: i32,
    ) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let mut enf = enf.lock().await;
            let enf = enf.deref_mut();
            user.has(enf, ROLE_MANAGER)?;
        }
        let it = ForumDao::by_id(db, id)?;
        if it.deleted_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::GONE, None)));
        }
        if it.locked_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
        }

        db.transaction::<_, Error, _>(|db| {
            ForumDao::update(db, it.id, &self.slug, &self.title, &self.description)?;
            Ok(())
        })?;

        Ok(())
    }
}
