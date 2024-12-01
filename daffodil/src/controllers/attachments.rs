use std::fs::File;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use actix_files::NamedFile;
use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, Responder, Result as WebResult};
use casbin::Enforcer;
use diesel::Connection as DieselConnection;
use futures_util::StreamExt;
use petunia::{
    graphql::Resource, jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool,
    rbac::v1 as rbac_v1, s3::Client as S3, session::Session, try_web, Error, Result,
};
use serde::Deserialize;
use tokio::sync::Mutex;

use super::super::{
    graphql::NAME,
    models::{
        attachment::{Dao as AttachmentDao, Item as Attachment},
        user::Item as User,
    },
    session::current_user,
};

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub resource: Resource,
    pub public: bool,
    pub expiration_days: Option<usize>,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "512MB")]
    pub file: TempFile,
    pub json: MPJson<Metadata>,
}

#[post("/")]
pub async fn upload(
    ss: Session,
    db: web::Data<DbPool>,
    jwt: web::Data<Jwt>,
    s3: web::Data<S3>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> WebResult<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let jwt = jwt.deref();
    let s3 = s3.deref();
    log::debug!("{:?}", form);
    let it = try_web!(form.execute(&ss, db, jwt, s3, NAME).await)?;
    Ok(web::Json(it))
}

impl UploadForm {
    pub async fn execute(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        s3: &S3,
        bucket: &str,
    ) -> Result<Attachment> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (_, user) = current_user(ss, db, jwt)?;
        let size = self.file.size;
        let content_type = self
            .file
            .content_type
            .as_ref()
            .unwrap_or(&mime::APPLICATION_OCTET_STREAM);
        let title = match self.file.file_name {
            Some(ref it) => it.clone(),
            None => "anonymous".to_string(),
        };
        let bucket = s3
            .create_bucket(bucket, self.json.public, self.json.expiration_days)
            .await?;
        let object = s3
            .upload_object(&bucket, &title, self.file.file.path())
            .await?;
        let it = db.transaction::<_, Error, _>(|db| {
            AttachmentDao::create(
                db,
                user.id,
                &bucket,
                &object,
                (&title, content_type, size as i32),
            )?;
            let it = AttachmentDao::by_bucket_and_object(db, &bucket, &object)?;
            AttachmentDao::set_upload_at(db, it.id)?;
            AttachmentDao::associate_(
                db,
                it.id,
                &self.json.resource.r#type,
                self.json.resource.id,
            )?;
            Ok(it)
        })?;
        Ok(it)
    }
}

#[get("/{token}")]
pub async fn show(
    ss: Session,
    db: web::Data<DbPool>,
    jwt: web::Data<Jwt>,
    s3: web::Data<S3>,
    enforcer: web::Data<Mutex<Enforcer>>,
    params: web::Path<(String, String, String)>,
) -> WebResult<impl Responder> {
    let (resource, bucket, object) = params.into_inner();
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.deref();
    let jwt = jwt.deref();
    let enf = enforcer.deref();
    let enf = enf.deref();
    let (_, user) = try_web!(current_user(&ss, db, jwt))?;
    let item = try_web!(AttachmentDao::by_bucket_and_object(db, &bucket, &object))?;

    try_web!(item.can_view(enf, &user, &resource).await)?;
    let tmp = try_web!(item.download(&s3).await)?;

    let content_type = try_web!(item.content_type.parse())?;
    // https://docs.rs/actix-files/latest/actix_files/struct.NamedFile.html#method.set_content_disposition
    Ok(NamedFile::open_async(&tmp)
        .await?
        .set_content_type(content_type))
}

impl Attachment {
    pub async fn download(&self, s3: &S3) -> Result<PathBuf> {
        let tmp = Path::new("tmp").join(format!("{}-{}", self.bucket, self.object));
        if !tmp.exists() {
            let url = s3.get_object_url(&self.bucket, &self.object, None).await?;
            let mut stream = reqwest::get(url).await?.bytes_stream();
            let mut file = File::create(&tmp)?;
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk)?;
            }
        }
        Ok(tmp)
    }

    pub async fn can_view(&self, enf: &Mutex<Enforcer>, user: &User, resource: &str) -> Result<()> {
        if user.id == self.user_id {
            return Ok(());
        }
        let resource = rbac_v1::policy_permissions_response::item::Resource::from_str(resource)?;
        let mut enf = enf.lock().await;
        let enf = enf.deref_mut();
        user.can(
            enf,
            &rbac_v1::policy_permissions_response::item::Operation::read(),
            &resource,
        )?;
        Ok(())
    }
}
