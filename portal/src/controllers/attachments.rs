use axum::{body::Body as HttpBody, extract::Multipart, http::Response as HttpResponse};
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use serde_json::Value as JsonValue;

use super::super::{
    Error, HttpError, Jwt, Result,
    cache::redis::StandaloneConnection as Cache,
    graphql::{CurrentUser, Session},
    models::attachment::{Dao as AttachmentDao, Item as Attachment},
    orm::postgresql::Connection as Db,
    s3::seaweedfs::Client as S3,
};

pub async fn show<J: Jwt>(
    db: &mut Db,
    jwt: &J,
    s3: &S3,
    token: &str,
    uid: &str,
    download: bool,
) -> Result<HttpResponse<HttpBody>> {
    let (bucket, object) = Attachment::from_uid(uid)?;
    let it = AttachmentDao::by_bucket_and_object(db, &bucket, &object)?;
    if !it.public {
        jwt.verify::<JsonValue>(token, CurrentUser::ISSUER, Attachment::AUDIENCE)
            .await?;
    }
    if it.deleted_at.is_some() {
        return Err(Box::new(HttpError(StatusCode::GONE, None)));
    }

    s3.show(&it.bucket, &it.object, &it.title, download || !it.online())
        .await
}
// The `Multipart` extractor must be the LAST argument in your handler if you use other extractors (like State or Json).
pub async fn upload<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    cache: &mut Cache,
    jwt: &J,
    s3: &S3,
    mut multipart: Multipart,
    public: bool,
) -> Result<()> {
    let current_user = ss.current_user(db, cache, jwt).await?;

    while let Some(mut field) = multipart.next_field().await? {
        let name = field
            .file_name()
            .ok_or_else(|| {
                Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("couldn't get field name".to_string()),
                ))
            })?
            .to_string();
        let content_type = Attachment::content_type(&name);
        let target = s3.assign().await?;
        log::info!("uploading file {} to {}/{}", name, target.url, target.fid);
        db.transaction::<_, Error, _>(|tx| {
            AttachmentDao::create(
                tx,
                current_user.id(),
                (&name, &content_type),
                (&target.url, &target.fid, public),
            )?;
            Ok(())
        })?;

        let mut size = 0;
        let mut index = 0;
        while let Some(chunk) = field.chunk().await? {
            let l = chunk.len();
            size += l;
            let buf = chunk.to_vec();
            s3.write(&name, &content_type, (buf, index), &target.url, &target.fid)
                .await?;
            log::debug!("uploaded {l} bytes for chunk-{index}");
            index += 1;
        }
        db.transaction::<_, Error, _>(|tx| {
            let it = AttachmentDao::by_bucket_and_object(tx, &target.url, &target.fid)?;
            AttachmentDao::set_uploaded_at(tx, it.id, size)?;
            Ok(())
        })?;
    }

    Ok(())
}
