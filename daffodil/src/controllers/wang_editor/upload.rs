use std::ops::{Deref, DerefMut};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, web, Responder};
use diesel::Connection as DieselConnection;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, s3::Client as S3,
    session::Session, Error, Result,
};
use serde::Serialize;

use super::super::super::{
    models::attachment::{Dao as AttachmentDao, Item as Attachment},
    session::current_user,
    NAME,
};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "512MB")]
    file: TempFile,
}

// https://www.wangeditor.com/en/v5/menu-config.html#server-address
#[derive(Debug, Serialize)]
pub struct Response {
    pub errno: i32,
    pub message: Option<String>,
    pub data: Option<Data>,
}

#[derive(Debug, Serialize)]
pub struct Data {
    pub url: String,
    pub alt: Option<String>,
    pub poster: Option<String>,
}

#[post("/image")]
pub async fn image(
    ss: Session,
    db: web::Data<DbPool>,
    jwt: web::Data<Jwt>,
    s3: web::Data<S3>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> impl Responder {
    let db = db.deref();
    let db = db.deref();
    let jwt = jwt.deref();
    let s3 = s3.deref();
    log::debug!("{:?}", form);
    let it = match form.image(&ss, db, jwt, s3, NAME).await {
        Ok(it) => it,
        Err(e) => Response {
            errno: 500,
            message: Some(e.to_string()),
            data: None,
        },
    };
    web::Json(it)
}

#[post("/video")]
pub async fn video(
    ss: Session,
    db: web::Data<DbPool>,
    jwt: web::Data<Jwt>,
    s3: web::Data<S3>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> impl Responder {
    let db = db.deref();
    let db = db.deref();
    let jwt = jwt.deref();
    let s3 = s3.deref();
    log::debug!("{:?}", form);
    let it = match form.video(&ss, db, jwt, s3, NAME).await {
        Ok(it) => it,
        Err(e) => Response {
            errno: 500,
            message: Some(e.to_string()),
            data: None,
        },
    };
    web::Json(it)
}

impl UploadForm {
    async fn image(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        s3: &S3,
        bucket: &str,
    ) -> Result<Response> {
        let (it, url) = self.execute(ss, db, jwt, s3, bucket).await?;
        Ok(Response {
            errno: 0,
            data: Some(Data {
                url,
                poster: None,
                alt: Some(it.title),
            }),
            message: None,
        })
    }
    async fn video(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        s3: &S3,
        bucket: &str,
    ) -> Result<Response> {
        let (_, url) = self.execute(ss, db, jwt, s3, bucket).await?;
        Ok(Response {
            errno: 0,
            data: Some(Data {
                url,
                poster: None,
                alt: None,
            }),
            message: None,
        })
    }
    async fn execute(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        s3: &S3,
        bucket: &str,
    ) -> Result<(Attachment, String)> {
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
        let bucket = s3.create_bucket(bucket, false, None).await?;
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
            Ok(it)
        })?;

        let url = format!("/api/attachments/{}", it.token(jwt, 100)?);
        Ok((it, url))
    }
}
