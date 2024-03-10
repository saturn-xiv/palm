use std::io::BufReader;
use std::ops::DerefMut;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, web, HttpResponse, Responder, Result as WebResult};
use mime::APPLICATION_OCTET_STREAM;
use palm::{minio::Connection as Minio, try_web, Result};
use serde::Deserialize;

use super::super::{
    models::{
        attachment::{Dao as AttachmentDao, Item as Attachment},
        user::Item as User,
    },
    orm::postgresql::{Connection as Db, Pool as DbPool},
};

#[derive(Deserialize)]
struct Query {
    bucket: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
}

#[post("/attachments")]
pub async fn create(
    user: User,
    query: web::Query<Query>,
    db: web::Data<DbPool>,
    s3: web::Data<Minio>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> WebResult<impl Responder> {
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();

    for it in form.files.iter() {
        // it.file.persist(path)?;
        try_web!(save(db, user.id, &s3, &query.bucket, it).await)?;
    }

    Ok(HttpResponse::Ok())
}

async fn save(db: &mut Db, user: i32, s3: &Minio, bucket: &str, file: &TempFile) -> Result<()> {
    let content_type = file
        .content_type
        .as_ref()
        .unwrap_or(&APPLICATION_OCTET_STREAM);
    let title = file.file_name.clone().unwrap_or("unknown".to_string());
    let name = Attachment::name(&title);
    let mut reader = BufReader::new(&file.file);
    s3.put_object(bucket, &name, content_type, &mut reader, file.size)
        .await?;
    AttachmentDao::create(
        db,
        user,
        bucket,
        &name,
        &title,
        content_type,
        file.size as u64,
    )?;
    Ok(())
}
