use std::ops::DerefMut;
use std::str::FromStr;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use language_tags::LanguageTag;
use petunia::{
    graphql::{Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    Error, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use crate::models::locale::I18n;

use super::super::{
    models::locale::{Dao as LocaleDao, Item as Locale},
    session::current_user,
};

#[derive(GraphQLObject)]
#[graphql(name = "Locale")]
pub struct Item {
    pub id: i32,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub updated_at: NaiveDateTime,
}

impl From<Locale> for Item {
    fn from(it: Locale) -> Self {
        Self {
            id: it.id,
            lang: it.lang.clone(),
            code: it.code.clone(),
            message: it.message.clone(),
            updated_at: it.updated_at,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "LocaleList")]
pub struct List {
    pub pagination: Pagination,
    pub items: Vec<Item>,
}

impl List {
    pub async fn new(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        pager: &Pager,
    ) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            user.is_administrator(enf)?;
        }

        let mut items = Vec::new();
        let total = LocaleDao::count(db)?;
        let pagination = Pagination::new(pager, total);
        for it in LocaleDao::index(db, pager.offset(total), pager.size())? {
            items.push(it.into());
        }
        Ok(Self { items, pagination })
    }
}

#[derive(Validate)]
pub struct Set {
    #[validate(length(min = 2, max = 15))]
    pub lang: String,
    #[validate(length(min = 2, max = 255))]
    pub code: String,
    #[validate(length(min = 1))]
    pub message: String,
}

impl Set {
    pub async fn execute(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
    ) -> Result<()> {
        self.validate()?;
        let lang = {
            let it = LanguageTag::from_str(&self.lang)?;
            it.to_string()
        };
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            user.is_administrator(enf)?;
        }

        db.transaction::<_, Error, _>(|db| {
            I18n::set(db, &lang, &self.code, &self.message)?;
            Ok(())
        })?;

        Ok(())
    }
}

impl Item {
    pub fn by_lang(db: &DbPool, lang: &str) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let mut items = Vec::new();
        for it in LocaleDao::by_lang(db, lang)? {
            items.push(it.into());
        }
        Ok(items)
    }
}
pub async fn destroy(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }

    db.transaction::<_, Error, _>(|db| {
        LocaleDao::destroy(db, id)?;
        Ok(())
    })?;

    Ok(())
}
