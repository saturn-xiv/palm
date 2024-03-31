use std::io::BufReader;
use std::ops::DerefMut;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, web, Responder, Result as WebResult};
use diesel::Connection as DieselConntection;
use mime::APPLICATION_OCTET_STREAM;
use palm::{minio::Connection as Minio, try_web, Error, Result};
use serde::Deserialize;

use super::super::{
    models::{
        attachment::{Dao as AttachmentDao, Item as Attachment},
        user::Item as User,
    },
    orm::postgresql::{Connection as Db, Pool as DbPool},
};

#[derive(Debug, Deserialize)]
struct Resource {
    r#type: Option<String>,
    id: Option<i32>,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
}

#[post("/attachments")]
pub async fn create(
    user: User,
    db: web::Data<DbPool>,
    s3: web::Data<Minio>,
    resource: web::Query<Resource>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> WebResult<impl Responder> {
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let resource = resource.into_inner();

    let mut items = Vec::new();
    for it in form.files.iter() {
        let it = try_web!(save(db, user.id, &s3, it, &resource).await)?;
        items.push(it);
    }

    Ok(web::Json(items))
}

async fn save(
    db: &mut Db,
    user: i32,
    s3: &Minio,
    file: &TempFile,
    resource: &Resource,
) -> Result<Attachment> {
    let content_type = file
        .content_type
        .as_ref()
        .unwrap_or(&APPLICATION_OCTET_STREAM);
    let title = file.file_name.clone().unwrap_or("unknown".to_string());
    let name = Attachment::name(&title);
    let mut reader = BufReader::new(&file.file);
    let bucket = s3.bucket(false, None).await?;
    s3.put_object(&bucket, &name, content_type, &mut reader, file.size)
        .await?;
    let it = db.transaction::<_, Error, _>(move |db| {
        AttachmentDao::create(
            db,
            user,
            &bucket,
            &name,
            &title,
            content_type,
            file.size as u64,
        )?;
        let it = AttachmentDao::by_bucket_and_name(db, &bucket, &name)?;
        if let Some(ref resource_type) = resource.r#type {
            if let Some(resource_id) = resource.id {
                AttachmentDao::associate_(db, it.id, resource_type, resource_id)?;
            }
        }

        Ok(it)
    })?;
    Ok(it)
}
