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
    models::category::{Dao as CategoryDao, Item as Category},
    session::current_user,
};

#[derive(GraphQLObject)]
#[graphql(name = "Category")]
pub struct Item {
    pub id: i32,
    pub code: String,
    pub left: i32,
    pub right: i32,
    pub updated_at: NaiveDateTime,
}

impl From<Category> for Item {
    fn from(it: Category) -> Self {
        Self {
            id: it.id,
            code: it.code.clone(),
            left: it.left,
            right: it.right,
            updated_at: it.updated_at,
        }
    }
}

impl Item {
    pub fn all(db: &DbPool) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let mut items = Vec::new();
        for it in CategoryDao::index(db)? {
            items.push(it.into());
        }
        Ok(items)
    }
    pub fn retrieving_full_tree(db: &DbPool, id: i32) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        let mut items = Vec::new();
        for it in CategoryDao::retrieving_full_tree(db, id)? {
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
            CategoryDao::create(db, &self.code, parent)?;
            Ok(())
        })?;

        Ok(())
    }
    pub async fn append(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        near: i32,
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
            CategoryDao::append(db, &self.code, near)?;
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
            let it = CategoryDao::by_id(db, id)?;
            CategoryDao::set_code(db, it.id, &self.code)?;
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
        CategoryDao::destroy(db, id)?;
        Ok(())
    })?;

    Ok(())
}
