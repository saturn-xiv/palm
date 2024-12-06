use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use actix_files::NamedFile;
use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, Responder, Result as WebResult};
use data_encoding::BASE64_NOPAD;
use diesel::Connection as DieselConnection;
use futures_util::StreamExt;
use petunia::{
    graphql::Resource,
    jwt::{openssl::OpenSsl as Jwt, Jwt as JwtProvider},
    orm::postgresql::Pool as DbPool,
    s3::Client as S3,
    session::Session,
    try_web, Error, Result,
};
use serde::{Deserialize, Serialize};

use super::super::{
    models::attachment::{Dao as AttachmentDao, Item as Attachment},
    session::current_user,
    NAME,
};

#[derive(Debug, Deserialize)]
struct Metadata {
    resource: Resource,
    public: Option<bool>,
    expiration_days: Option<usize>,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "512MB")]
    file: TempFile,
    json: MPJson<Metadata>,
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
            .create_bucket(
                bucket,
                self.json.public.unwrap_or(false),
                self.json.expiration_days,
            )
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

#[derive(Debug, Serialize, Deserialize)]
struct Subject {
    bucket: String,
    object: String,
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = {
            let buf = flexbuffers::to_vec(self).map_err(|e| {
                log::error!("{:?}", e);
                fmt::Error
            })?;
            BASE64_NOPAD.encode(&buf)
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Subject {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let buf = BASE64_NOPAD.decode(s.as_bytes())?;
        let it = flexbuffers::from_slice(&buf[..])?;
        Ok(it)
    }
}

impl Subject {
    pub const AUDIENCE: &str = "attachment.show";
}

#[get("/{token}")]
pub async fn show(
    db: web::Data<DbPool>,
    jwt: web::Data<Jwt>,
    s3: web::Data<S3>,
    params: web::Path<(String,)>,
) -> WebResult<impl Responder> {
    let subject: Subject = {
        let (token,) = params.into_inner();
        let jwt = jwt.deref();
        let jwt = jwt.deref();
        let it = try_web!(jwt.verify(&token, Subject::AUDIENCE))?;
        try_web!(it.parse())?
    };

    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();

    let item = try_web!(AttachmentDao::by_bucket_and_object(
        db,
        &subject.bucket,
        &subject.object
    ))?;

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
            let url = self.url(s3, None).await?;
            let mut stream = reqwest::get(url).await?.bytes_stream();
            let mut file = File::create(&tmp)?;
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk)?;
            }
        }
        Ok(tmp)
    }
    pub fn token(&self, jwt: &Jwt, years: i32) -> Result<String> {
        let (nbf, exp) = Jwt::years(years)?;
        let subject = Subject {
            bucket: self.bucket.clone(),
            object: self.object.clone(),
        }
        .to_string();
        let it = jwt.sign(&subject, Subject::AUDIENCE, nbf, exp)?;
        Ok(it)
    }
}
