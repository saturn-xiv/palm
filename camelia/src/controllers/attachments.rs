use std::ops::Deref;
use std::ops::DerefMut;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, web, Responder, Result as WebResult};
use chrono::Duration;
use diesel::Connection as DieselConntection;
use mime::APPLICATION_OCTET_STREAM;
use palm::{jasmine::S3, try_web, Error, Result};
use serde::Deserialize;
use tokio::fs::File as TokioFile;

use super::super::{
    models::{
        attachment::{Dao as AttachmentDao, Item as Attachment},
        user::Item as User,
    },
    orm::postgresql::{Connection as Db, Pool as DbPool},
};
use super::Jasmine;

#[derive(Debug, Deserialize)]
struct Resource {
    r#type: Option<String>,
    id: Option<i64>,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
}

#[post("/attachments/{bucket}")]
pub async fn create(
    user: User,
    path: web::Path<(String,)>,
    db: web::Data<DbPool>,
    s3: web::Data<Jasmine>,
    resource: web::Query<Resource>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> WebResult<impl Responder> {
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let resource = resource.into_inner();
    let (bucket,) = path.into_inner();
    let s3 = s3.into_inner();
    let s3 = s3.deref();

    let mut items = Vec::new();
    for it in form.files.iter() {
        let it = try_web!(save(db, user.id, &s3.0, &bucket, it, &resource).await)?;
        items.push(it);
    }

    Ok(web::Json(items))
}

async fn save<S: S3>(
    db: &mut Db,
    user: i64,
    s3: &S,
    bucket: &str,
    file: &TempFile,
    resource: &Resource,
) -> Result<Attachment> {
    let content_type = file
        .content_type
        .as_ref()
        .unwrap_or(&APPLICATION_OCTET_STREAM);
    let title = file.file_name.clone().unwrap_or("unknown".to_string());
    let name = Attachment::name(&title);

    // https://min.io/docs/minio/linux/integrations/presigned-put-upload-via-browser.html
    {
        // let mut reader = BufReader::new(&file.file);
        let file = file.file.as_ref();
        let file = TokioFile::open(file).await?;
        let url = s3.upload_file(bucket, &name, Duration::hours(1))?;
        let cli = reqwest::Client::new();
        cli.put(&url).body(file).send().await?;
    }
    let it = db.transaction::<_, Error, _>(move |db| {
        AttachmentDao::create(
            db,
            user,
            bucket,
            &name,
            &title,
            content_type,
            file.size as u64,
        )?;
        let it = AttachmentDao::by_bucket_and_name(db, bucket, &name)?;
        if let Some(ref resource_type) = resource.r#type {
            if let Some(resource_id) = resource.id {
                AttachmentDao::associate_(db, it.id, resource_type, resource_id)?;
            }
        }

        Ok(it)
    })?;
    Ok(it)
}
