use std::ops::DerefMut;
use std::str::FromStr;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use language_tags::LanguageTag;
use petunia::{
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    session::Session,
    Error, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::menu::{Dao as MenuDao, Item as MenuItem},
    session::current_user,
};

#[derive(GraphQLObject)]
#[graphql(name = "Menu")]
pub struct Menu {
    pub id: i32,
    pub label: String,
    pub sort_order: i32,
    pub children: Vec<Self>,
}

impl Menu {
    pub fn all(db: &DbPool, lang: &str, location: &str) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let mut items = Vec::new();
        for it in MenuDao::root(db, lang, location)?.iter() {
            items.push(Self {
                id: it.id,
                label: it.label.clone(),
                sort_order: it.sort_order,
                children: Self::children(db, it.id)?,
            });
        }
        Ok(items)
    }
    fn children(db: &mut Db, id: i32) -> Result<Vec<Self>> {
        let mut items = Vec::new();
        for it in MenuDao::children(db, id)?.iter() {
            items.push(Self {
                id: it.id,
                label: it.label.clone(),
                sort_order: it.sort_order,
                children: Self::children(db, it.id)?,
            });
        }
        Ok(items)
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "MenuItem")]
pub struct Item {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub lang: String,
    pub location: String,
    pub label: String,
    pub sort_order: i32,
    pub updated_at: NaiveDateTime,
}

impl From<MenuItem> for Item {
    fn from(it: MenuItem) -> Self {
        Self {
            id: it.id,
            parent_id: it.parent_id,
            lang: it.lang.clone(),
            location: it.location.clone(),
            label: it.label.clone(),
            sort_order: it.sort_order,
            updated_at: it.updated_at,
        }
    }
}
impl Item {
    pub fn all(db: &DbPool) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let mut items = Vec::new();
        for it in MenuDao::index(db)? {
            items.push(it.into());
        }
        Ok(items)
    }
}

#[derive(Validate)]
pub struct Form {
    #[validate(length(min = 2, max = 63))]
    pub label: String,
    #[validate(range(min = -99, max = 99 ))]
    pub sort_order: i32,
}

impl Form {
    pub async fn update(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        id: i32,
    ) -> Result<()> {
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
            MenuDao::update(db, id, &self.label, self.sort_order)?;
            Ok(())
        })?;

        Ok(())
    }
    pub async fn create(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        parent: i32,
    ) -> Result<()> {
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
            MenuDao::create(db, parent, &self.label, self.sort_order)?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Validate)]
pub struct Append {
    #[validate(length(min = 2, max = 15))]
    pub lang: String,
    #[validate(length(min = 2, max = 31))]
    pub location: String,
    #[validate(length(min = 2, max = 63))]
    pub label: String,
    #[validate(range(min = -99, max = 99 ))]
    pub sort_order: i32,
}

impl Append {
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
            MenuDao::append(db, &lang, &self.location, &self.label, self.sort_order)?;
            Ok(())
        })?;

        Ok(())
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
        MenuDao::destroy(db, id)?;
        Ok(())
    })?;

    Ok(())
}
