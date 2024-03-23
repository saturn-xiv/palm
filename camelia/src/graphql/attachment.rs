use casbin::Enforcer;
use chrono::NaiveDateTime;
use hyper::StatusCode;
use juniper::GraphQLObject;
use log::warn;
use palm::{
    cache::redis::ClusterConnection as Cache,
    jwt::Jwt,
    minio::Connection as Minio,
    pagination::{Pager, Pagination},
    rbac::Operation,
    session::Session,
    HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::{
        attachment::{Dao as AttachmentDao, Item as Attachment},
        user::Item as User,
    },
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};

#[derive(GraphQLObject)]
#[graphql(name = "AttachmentIndexResponseItem")]
pub struct IndexResponseItem {
    pub id: i32,
    pub bucket: String,
    pub name: String,
    pub title: String,
    pub size: i32,
    pub content_type: String,
    pub status: String,
    pub updated_at: NaiveDateTime,
}

impl IndexResponseItem {
    pub fn pictures<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        jwt: &J,
    ) -> Result<Vec<Self>> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        let items = AttachmentDao::pictures_by_user(db, user.id)?
            .into_iter()
            .map(|x| x.into())
            .collect();
        Ok(items)
    }
}

impl From<Attachment> for IndexResponseItem {
    fn from(x: Attachment) -> Self {
        Self {
            id: x.id,
            bucket: x.bucket.clone(),
            name: x.name.clone(),
            title: x.title.clone(),
            size: x.size as i32,
            content_type: x.content_type.clone(),
            status: x.status.clone(),
            updated_at: x.updated_at,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "AttachmentIndexResponse")]
pub struct IndexResponse {
    pub items: Vec<IndexResponseItem>,
    pub pagination: Pagination,
}

impl IndexResponse {
    pub fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        jwt: &J,
        pager: &Pager,
    ) -> Result<Self> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        let total = AttachmentDao::count_by_user(db, user.id)?;
        let items = AttachmentDao::by_user(db, user.id, pager.offset(total), pager.size())?
            .into_iter()
            .map(|x| x.into())
            .collect();

        Ok(Self {
            items,
            pagination: Pagination::new(pager, total),
        })
    }
}

#[derive(Validate)]
pub struct ByBucketAndNameRequest {
    #[validate(length(min = 1, max = 63))]
    pub bucket: String,
    #[validate(length(min = 1, max = 31))]
    pub name: String,
    #[validate(range(min = 60, max = 604800))]
    pub ttl: Option<i64>,
}

impl ByBucketAndNameRequest {
    pub async fn handle<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        s3: &Minio,
    ) -> Result<ShowResponse> {
        self.validate()?;
        let item = AttachmentDao::by_bucket_and_name(db, &self.bucket, &self.name)?;
        ShowResponse::new(ss, db, ch, enf, jwt, s3, (&item, self.ttl)).await
    }
}

#[derive(Validate)]
pub struct ByIdRequest {
    #[validate(range(min = 1))]
    pub id: i32,
    #[validate(range(min = 60, max = 604800))]
    pub ttl: Option<i64>,
}

impl ByIdRequest {
    pub async fn handle<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        s3: &Minio,
    ) -> Result<ShowResponse> {
        self.validate()?;
        let item = AttachmentDao::by_id(db, self.id)?;
        ShowResponse::new(ss, db, ch, enf, jwt, s3, (&item, self.ttl)).await
    }
}
#[derive(GraphQLObject)]
#[graphql(name = "AttachmentShowResponse")]
pub struct ShowResponse {
    pub url: String,
    pub title: String,
    pub size: i32,
    pub content_type: String,
    pub updated_at: NaiveDateTime,
}

impl ShowResponse {
    async fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        s3: &Minio,
        (item, ttl): (&Attachment, Option<i64>),
    ) -> Result<Self> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        item.can_delete(enf, &user).await?;
        let url = item.url(s3, ttl).await?;

        Ok(Self {
            url,
            title: item.title.clone(),
            size: item.size as i32,
            content_type: item.content_type.clone(),
            updated_at: item.updated_at,
        })
    }
}

pub async fn destroy<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    id: i32,
) -> Result<()> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    let item = AttachmentDao::by_id(db, id)?;
    item.can_delete(enf, &user).await?;
    warn!("delete attachment {item}");
    AttachmentDao::delete(db, id)?;
    Ok(())
}

impl Attachment {
    pub async fn can_show(&self, enf: &Mutex<Enforcer>, user: &User) -> Result<()> {
        if self.deleted_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::NOT_FOUND, None)));
        }
        if self.user_id == user.id {
            return Ok(());
        }
        if user.is_administrator(enf).await.is_ok() {
            return Ok(());
        }
        if user
            .can::<Self, _>(enf, Operation::Show, Some(self.id))
            .await
            .is_ok()
        {
            return Ok(());
        }
        if user
            .can::<Self, _>(enf, Operation::Edit, Some(self.id))
            .await
            .is_ok()
        {
            return Ok(());
        }
        Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
    }

    pub async fn can_delete(&self, enf: &Mutex<Enforcer>, user: &User) -> Result<()> {
        if self.deleted_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::NOT_FOUND, None)));
        }
        if self.user_id == user.id {
            return Ok(());
        }
        if user.is_administrator(enf).await.is_ok() {
            return Ok(());
        }
        if user
            .can::<Self, _>(enf, Operation::Delete, Some(self.id))
            .await
            .is_ok()
        {
            return Ok(());
        }

        Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
    }
}
