use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use chrono::Duration;
use diesel::Connection as DieselConntection;
use hibiscus::cache::redis::Pool as CachePool;
use hyper::StatusCode;
use palm::{
    azalea::v1::{IdRequest, Pager, Pagination},
    camelia::v1,
    gourd::Policy,
    jasmine::S3,
    to_chrono_duration, to_timestamp, try_grpc, Error, GrpcResult, HttpError, Result, Thrift,
};
use tonic::{Request, Response, Status};
use validator::Validate;

use super::super::{
    models::{
        attachment::{Bucket, Dao as AttachmentDao, Item as Attachment},
        user::Item as User,
    },
    orm::postgresql::{Connection as Db, Pool as DbPool},
};
use super::{CurrentUserAdapter, Session};

pub struct Service {
    pub loquat: Arc<Thrift>,
    pub gourd: Arc<Thrift>,
    pub jasmine: Arc<Thrift>,
    pub db: DbPool,
    pub cache: CachePool,
}

#[tonic::async_trait]
impl v1::attachment_server::Attachment for Service {
    async fn upload_url(
        &self,
        req: Request<v1::AttachmentUploadUrlRequest>,
    ) -> GrpcResult<v1::AttachmentUploadUrlResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let s3 = self.jasmine.deref();

        let req = req.into_inner();
        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;

        let url = try_grpc!(upload(
            db,
            s3,
            user.id,
            &Bucket::new(req.public, req.expiration_days),
            &req.title,
            &req.content_type,
            req.size
        ))?;
        Ok(Response::new(v1::AttachmentUploadUrlResponse { url }))
    }
    async fn index(&self, req: Request<Pager>) -> GrpcResult<v1::AttachmentIndexResponse> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();

        let pager = req.into_inner();
        let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;

        let total = try_grpc!(AttachmentDao::count_by_user(db, user.id))?;
        let items = try_grpc!(AttachmentDao::by_user(
            db,
            user.id,
            pager.offset(total),
            pager.size()
        ))?;
        Ok(Response::new(v1::AttachmentIndexResponse {
            items: items.into_iter().map(|x| x.into()).collect(),
            pagination: Some(Pagination::new(&pager, total)),
        }))
    }
    async fn publish(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        let it = try_grpc!(AttachmentDao::by_id(db, req.id))?;
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(can_edit(policy, &user, &it))?;
        }

        try_grpc!(AttachmentDao::publish(db, it.id))?;
        Ok(Response::new(()))
    }
    async fn destroy(&self, req: Request<IdRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        let it = try_grpc!(AttachmentDao::by_id(db, req.id))?;
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(can_edit(policy, &user, &it))?;
        }

        {
            let items = try_grpc!(AttachmentDao::resources(db, it.id))?;
            if !items.is_empty() {
                return Err(Status::invalid_argument(format!(
                    "attachment {} is in use.",
                    it.title
                )));
            }
        }
        try_grpc!(AttachmentDao::delete(db, it.id))?;
        Ok(Response::new(()))
    }
    async fn show(
        &self,
        req: Request<v1::AttachmentShowRequest>,
    ) -> GrpcResult<v1::AttachmentShowResponse> {
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let s3 = self.jasmine.deref();

        let req = req.into_inner();

        let it = try_grpc!(AttachmentDao::by_id(db, req.id))?;
        let url = try_grpc!(it.show(s3, req.ttl.map(|x| to_chrono_duration!(x))))?;
        Ok(Response::new(v1::AttachmentShowResponse { url }))
    }
    async fn update(&self, req: Request<v1::AttachmentUpdateRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        let it = try_grpc!(AttachmentDao::by_id(db, req.id))?;
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(can_edit(policy, &user, &it))?;
        }

        {
            let form = TitleForm { value: &req.title };
            try_grpc!(form.validate())?;
        }
        try_grpc!(AttachmentDao::set_title(db, it.id, &req.title))?;
        Ok(Response::new(()))
    }
    async fn associate(&self, req: Request<v1::AttachmentResourceRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        let it = try_grpc!(AttachmentDao::by_id(db, req.id))?;
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(can_edit(policy, &user, &it))?;
        }

        try_grpc!(AttachmentDao::associate_(
            db,
            it.id,
            &req.resource_type,
            req.resource_id
        ))?;
        Ok(Response::new(()))
    }
    async fn dissociate(&self, req: Request<v1::AttachmentResourceRequest>) -> GrpcResult<()> {
        let ss = Session::new(&req);
        let mut db = try_grpc!(self.db.get())?;
        let db = db.deref_mut();
        let mut ch = try_grpc!(self.cache.get())?;
        let ch = ch.deref_mut();
        let jwt = self.loquat.deref();
        let policy = self.gourd.deref();
        let req = req.into_inner();

        let it = try_grpc!(AttachmentDao::by_id(db, req.id))?;
        {
            let (user, _, _) = try_grpc!(ss.current_user(db, ch, jwt))?;
            try_grpc!(can_edit(policy, &user, &it))?;
        }

        try_grpc!(AttachmentDao::dissociate_(
            db,
            it.id,
            &req.resource_type,
            req.resource_id
        ))?;
        Ok(Response::new(()))
    }
}

fn can_edit<P: Policy>(policy: &P, user: &User, attachment: &Attachment) -> Result<()> {
    if attachment.user_id == user.id {
        return Ok(());
    }
    if user.is_administrator(policy).is_ok() {
        return Ok(());
    }
    Err(Box::new(HttpError(
        StatusCode::FORBIDDEN,
        Some(format!("couldn't edit {}", attachment.title)),
    )))
}

impl From<Attachment> for v1::attachment_index_response::Item {
    fn from(x: Attachment) -> Self {
        Self {
            id: x.id,
            bucket: x.bucket.clone(),
            name: x.name.clone(),
            title: x.title.clone(),
            content_type: x.content_type.clone(),
            size: x.size,
            deleted_at: x.deleted_at.map(|x| to_timestamp!(x)),
            published_at: x.published_at.map(|x| to_timestamp!(x)),
            updated_at: Some(to_timestamp!(x.updated_at)),
        }
    }
}

#[derive(Validate)]
pub struct TitleForm<'a> {
    #[validate(length(min = 1, max = 127))]
    pub value: &'a str,
}

fn upload<S: S3>(
    db: &mut Db,
    s3: &S,
    user: i64,
    bucket: &Bucket,
    title: &str,
    content_type: &str,
    size: u64,
) -> Result<String> {
    let bucket = {
        let name = bucket.to_string();
        s3.create_bucket(
            &name,
            bucket.public,
            bucket.expiration_days.unwrap_or_default(),
        )?;
        name
    };
    let object = Bucket::object(title);
    let content_type = content_type.parse()?;
    let url = s3.upload_file(&bucket, &object, Duration::hours(1))?;
    db.transaction::<_, Error, _>(move |db| {
        AttachmentDao::create(db, user, &bucket, &object, title, &content_type, size)?;
        Ok(())
    })?;

    Ok(url)
}

impl Attachment {
    pub fn show<S: S3>(self, s3: &S, ttl: Option<Duration>) -> Result<String> {
        let bucket: Bucket = self.bucket.parse()?;
        let url = if bucket.public {
            s3.get_presigned_url(
                &self.bucket,
                &self.name,
                &self.title,
                ttl.unwrap_or(Duration::days(7)),
            )?
        } else {
            s3.get_permanent_url(&self.bucket, &self.name)?
        };
        Ok(url)
    }
}
