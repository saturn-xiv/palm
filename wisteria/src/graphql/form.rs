use std::{ops::DerefMut, str::FromStr};

use chrono::NaiveDateTime;
use daffodil::{models::user::Item as User, session::current_user};
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, session::Session, Editor, Error,
    HttpError, Result,
};
use validator::Validate;

use super::super::models::form::{Dao as FormDao, Item as Form};

#[derive(GraphQLObject)]
#[graphql(name = "QuestionnaireForm")]
pub struct Item {
    pub id: i32,
    pub uid: String,
    pub title: String,
    pub description: String,
    pub description_editor: String,
    pub status: String,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Form> for Item {
    fn from(it: Form) -> Self {
        Self {
            id: it.id,
            uid: it.uid.clone(),
            title: it.title.clone(),
            description: it.description.clone(),
            description_editor: it.description_editor.clone(),
            status: it.status.clone(),
            locked_at: it.locked_at,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        }
    }
}

impl Item {
    pub fn all(ss: &Session, db: &DbPool, jwt: &Jwt) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;

        let mut items = Vec::new();
        for it in FormDao::by_user(db, user.id)? {
            items.push(it.into());
        }
        Ok(items)
    }
}

#[derive(Validate)]
pub struct Create {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(length(min = 1, max = 15))]
    pub editor: String,
}

impl Create {
    pub fn execute(&self, ss: &Session, db: &DbPool, jwt: &Jwt) -> Result<()> {
        self.validate()?;
        let editor = Editor::from_str(&self.editor)?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
        db.transaction::<_, Error, _>(|db| {
            FormDao::create(db, user.id, &self.title, &self.description, editor)?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Validate)]
pub struct Update {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1))]
    pub description: String,
}

impl Update {
    pub fn execute(&self, ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
        let it = FormDao::by_id(db, id)?;
        it.can_edit(&user)?;
        db.transaction::<_, Error, _>(|db| {
            FormDao::update(db, it.id, &self.title, &self.description)?;
            Ok(())
        })?;
        Ok(())
    }
}
pub fn enable(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let it = FormDao::by_id(db, id)?;
    it.can_edit(&user)?;
    db.transaction::<_, Error, _>(|db| {
        FormDao::enable(db, it.id)?;
        Ok(())
    })?;
    Ok(())
}
pub fn disable(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let it = FormDao::by_id(db, id)?;
    it.can_edit(&user)?;
    db.transaction::<_, Error, _>(|db| {
        FormDao::disable(db, it.id)?;
        Ok(())
    })?;
    Ok(())
}
pub fn lock(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let it = FormDao::by_id(db, id)?;
    it.can_edit(&user)?;
    db.transaction::<_, Error, _>(|db| {
        FormDao::lock(db, it.id)?;
        Ok(())
    })?;
    Ok(())
}
pub fn unlock(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
    let it = FormDao::by_id(db, id)?;
    it.can_edit(&user)?;
    db.transaction::<_, Error, _>(|db| {
        FormDao::unlock(db, it.id)?;
        Ok(())
    })?;
    Ok(())
}

impl Form {
    pub fn can_edit(&self, user: &User) -> Result<()> {
        if self.user_id != user.id {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
        Ok(())
    }
}
