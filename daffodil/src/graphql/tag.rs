use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, session::Session, Error, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::tag::{Dao as TagDao, Item as Tag},
    session::current_user,
};

#[derive(GraphQLObject)]
#[graphql(name = "Tag")]
pub struct Item {
    pub id: i32,
    pub code: String,
    pub updated_at: NaiveDateTime,
}

impl From<Tag> for Item {
    fn from(it: Tag) -> Self {
        Self {
            id: it.id,
            code: it.code.clone(),
            updated_at: it.updated_at,
        }
    }
}

impl Item {
    pub fn all(db: &DbPool) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let mut items = Vec::new();
        for it in TagDao::index(db)? {
            items.push(it.into());
        }
        Ok(items)
    }
}

#[derive(Validate)]
pub struct Form {
    #[validate(length(min = 2, max = 255))]
    pub code: String,
}

impl Form {
    pub async fn create(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
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
            TagDao::create(db, &self.code)?;
            Ok(())
        })?;

        Ok(())
    }
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
            let it = TagDao::by_id(db, id)?;
            TagDao::set_code(db, it.id, &self.code)?;
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
        TagDao::destroy(db, id)?;
        Ok(())
    })?;

    Ok(())
}
