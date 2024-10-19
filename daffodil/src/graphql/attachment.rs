use std::ops::DerefMut;

use chrono::NaiveDateTime;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use petunia::{
    graphql::{Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    Error, HttpError, Result,
};
use validator::Validate;

use super::super::models::{
    attachment::{Dao as AttachmentDao, Item as Attachment},
    user::Item as User,
};

#[derive(GraphQLObject)]
#[graphql(name = "Attachment")]
pub struct Item {
    pub id: i32,
    pub bucket: String,
    pub object: String,
    pub title: String,
    pub size: i32,
    pub content_type: String,
    pub uploaded_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Attachment> for Item {
    fn from(it: Attachment) -> Self {
        Self {
            id: it.id,
            bucket: it.bucket.clone(),
            object: it.object.clone(),
            title: it.title.clone(),
            size: it.size,
            content_type: it.content_type.clone(),
            uploaded_at: it.uploaded_at,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "AttachmentList")]
pub struct List {
    pub pagination: Pagination,
    pub items: Vec<Item>,
}

impl List {
    pub fn new(ss: &Session, db: &DbPool, jwt: &Jwt, pager: &Pager) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let user = User::new(ss, db, jwt)?;

        let mut items = Vec::new();
        let total = AttachmentDao::count_by_user(db, user.id)?;
        let pagination = Pagination::new(pager, total);
        for it in AttachmentDao::by_user(db, user.id, pager.offset(total), pager.size())? {
            items.push(it.into());
        }
        Ok(Self { items, pagination })
    }
}

pub async fn destroy(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let user = User::new(ss, db, jwt)?;
    let it = AttachmentDao::by_id(db, id)?;
    if it.user_id != user.id {
        return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
    }
    db.transaction::<_, Error, _>(|db| {
        AttachmentDao::disable(db, id)?;
        Ok(())
    })?;

    Ok(())
}

pub async fn set_uploaded_at(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let user = User::new(ss, db, jwt)?;
    let it = AttachmentDao::by_id(db, id)?;
    if it.user_id != user.id {
        return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
    }

    db.transaction::<_, Error, _>(|db| {
        AttachmentDao::set_upload_at(db, id)?;
        Ok(())
    })?;

    Ok(())
}

#[derive(Validate)]
pub struct SetTitle {
    #[validate(length(min = 1, max = 63))]
    pub title: String,
}

impl SetTitle {
    pub fn execute(&self, ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let user = User::new(ss, db, jwt)?;
        let it = AttachmentDao::by_id(db, id)?;
        if it.user_id != user.id {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
        db.transaction::<_, Error, _>(|db| {
            AttachmentDao::set_title(db, id, &self.title)?;
            Ok(())
        })?;
        Ok(())
    }
}
