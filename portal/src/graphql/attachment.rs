use chrono::NaiveDateTime;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;

use super::super::{
    Error, HttpError, Jwt, Result,
    cache::redis::StandaloneConnection as Cache,
    models::attachment::{Dao as AttachmentDao, Item as AttachmentItem},
    orm::postgresql::Connection as Db,
    rbac::Rbac,
};
use super::{Page, Pagination, Session};

pub async fn destroy<R: Rbac, J: Jwt>(
    ss: &Session,
    db: &mut Db,
    cache: &mut Cache,
    rbac: &R,
    jwt: &J,
    id: i32,
) -> Result<()> {
    let current_user = ss.current_user(db, cache, jwt).await?;
    let it = can_write(db, rbac, current_user.id(), id as i64).await?;
    db.transaction::<_, Error, _>(|tx| {
        AttachmentDao::delete(tx, it.id)?;
        Ok(())
    })?;

    Ok(())
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "IndexAttachmentResponse")]
pub struct Index {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}
impl Index {
    pub async fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        jwt: &J,
        page: &Page,
    ) -> Result<Self> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        let total = AttachmentDao::count_by_user(db, current_user.id())?;
        let items =
            AttachmentDao::index_by_user(db, current_user.id(), page.offset(total), page.size())?;

        Ok(Self {
            items: items.into_iter().map(|x| x.into()).collect(),
            pagination: Pagination::new(page, total),
        })
    }
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "Attachment")]
pub struct Item {
    pub id: i32,
    pub bucket: String,
    pub object: String,
    pub title: String,
    pub size: Option<i32>,
    pub content_type: String,
    pub public: bool,
    pub uploaded_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
}

impl From<AttachmentItem> for Item {
    fn from(it: AttachmentItem) -> Self {
        Self {
            id: it.id as i32,
            bucket: it.bucket.clone(),
            object: it.object.clone(),
            title: it.title.clone(),
            size: it.size.map(|x| x as i32),
            content_type: it.content_type.clone(),
            public: it.public,
            uploaded_at: it.uploaded_at,
            deleted_at: it.deleted_at,
            version: it.version,
            updated_at: it.updated_at,
        }
    }
}

impl Item {
    pub async fn new<R: Rbac, J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        id: i32,
    ) -> Result<Self> {
        let current_user = ss.current_user(db, cache, jwt).await?;
        let it = can_read(db, rbac, current_user.id(), id as i64).await?;
        Ok(it.into())
    }
}

async fn can_read<R: Rbac>(db: &mut Db, rbac: &R, user: i64, id: i64) -> Result<AttachmentItem> {
    let it = AttachmentDao::by_id(db, id)?;
    if it.user_id == user
        || rbac.can_read::<AttachmentItem>(user, it.id).await.is_ok()
        || rbac.is_administrator(user).await.is_ok()
    {
        return Ok(it);
    }
    Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
}

async fn can_write<R: Rbac>(db: &mut Db, rbac: &R, user: i64, id: i64) -> Result<AttachmentItem> {
    let it = AttachmentDao::by_id(db, id)?;
    if it.user_id == user
        || rbac.can_read::<AttachmentItem>(user, it.id).await.is_ok()
        || rbac.is_administrator(user).await.is_ok()
    {
        return Ok(it);
    }
    Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
}
