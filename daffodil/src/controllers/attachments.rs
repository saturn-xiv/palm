use std::ops::{Deref, DerefMut};

use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};
use actix_web::{
    error::ErrorInternalServerError, post, web, Error as WebError, Responder, Result as WebResult,
};
use diesel::Connection as DieselConnection;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, s3::Client as S3,
    session::Session, Error, Result,
};
use serde::Deserialize;

use super::super::{
    graphql::NAME,
    models::{
        attachment::{Dao as AttachmentDao, Item as Attachment},
        user::Item as User,
    },
};

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "512MB")]
    pub file: TempFile,
    pub json: MPJson<Metadata>,
}

#[post("/upload")]
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
    let it = form
        .execute(&ss, db, jwt, s3, (NAME, false, -1))
        .await
        .map_err(|e| -> WebError { ErrorInternalServerError(e) })?;
    Ok(web::Json(it))
}

impl UploadForm {
    pub async fn execute(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        s3: &S3,
        (bucket, public, expiration_days): (&str, bool, i32),
    ) -> Result<Attachment> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        let user = User::new(ss, db, jwt)?;
        let size = self.file.size;
        let content_type = self
            .file
            .content_type
            .as_ref()
            .unwrap_or(&mime::APPLICATION_OCTET_STREAM);
        let bucket = s3.create_bucket(bucket, public, expiration_days).await?;
        let (_, object, _, _) = s3.upload_object(&bucket, self.file.file.path()).await?;
        let it = db.transaction::<_, Error, _>(|db| {
            AttachmentDao::create(
                db,
                user.id,
                &bucket,
                &object,
                (&self.json.name, content_type, size as i32),
            )?;
            let it = AttachmentDao::by_bucket_and_object(db, &bucket, &object)?;
            AttachmentDao::set_upload_at(db, it.id)?;
            Ok(it)
        })?;
        Ok(it)
    }
}
