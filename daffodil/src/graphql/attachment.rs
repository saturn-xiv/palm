use std::ops::DerefMut;

use chrono::{Duration, NaiveDateTime};
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use petunia::{
    graphql::{Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    s3::Client as Minio,
    session::Session,
    Error, HttpError, Result,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::super::{
    models::attachment::{Dao as AttachmentDao, Item as Attachment},
    session::current_user,
};

#[derive(GraphQLObject, Deserialize, Serialize, Debug, Clone)]
#[graphql(name = "Attachment")]
pub struct Item {
    pub id: i32,
    pub bucket: String,
    pub object: String,
    pub title: String,
    pub size: i32,
    pub content_type: String,
    pub url: String,
    pub uploaded_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub async fn new(s3: &Minio, it: &Attachment, ttl: Option<Duration>) -> Result<Self> {
        let it = Self {
            id: it.id,
            bucket: it.bucket.clone(),
            object: it.object.clone(),
            title: it.title.clone(),
            size: it.size,
            content_type: it.content_type.clone(),
            url: it.url(s3, ttl).await?,
            uploaded_at: it.uploaded_at,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        };
        Ok(it)
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "AttachmentList")]
pub struct List {
    pub pagination: Pagination,
    pub items: Vec<Item>,
}

impl List {
    pub async fn new(
        ss: &Session,
        db: &DbPool,
        s3: &Minio,
        jwt: &Jwt,
        pager: &Pager,
        ttl: Option<Duration>,
    ) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;

        let mut items = Vec::new();
        let total = AttachmentDao::count_by_user(db, user.id)?;
        let pagination = Pagination::new(pager, total);
        for it in AttachmentDao::by_user(db, user.id, pager.offset(total), pager.size())?.iter() {
            items.push(Item::new(s3, it, ttl).await?);
        }
        Ok(Self { items, pagination })
    }
}

pub fn destroy(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
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

pub fn set_uploaded_at(ss: &Session, db: &DbPool, jwt: &Jwt, id: i32) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let (_, user) = current_user(ss, db, jwt)?;
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
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
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
