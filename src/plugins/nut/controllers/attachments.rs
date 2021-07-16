use std::ops::Deref;
use std::sync::Arc;

use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};

use super::super::super::super::{request::Token, Error, HttpResult};
use super::super::models::attachment::{Dao as AttachmentDao, Item as Attachment};
use super::State;

#[post("/attachments/")]
pub async fn create(
    token: Token,
    mut payload: Multipart,
    state: web::Data<Arc<State>>,
) -> HttpResult<impl Responder> {
    let db = state.db.get().map_err(Error::from)?;
    let db = db.deref();
    let jwt = state.jwt.deref();
    let s3 = state.s3.deref();
    let user = token.current_user(db, jwt)?;
    while let Ok(Some(mut field)) = payload.try_next().await {
        if let Some(title) = field.content_disposition() {
            if let Some(title) = title.get_filename() {
                info!("receive file {}", title);
                let (bucket, name, content_type) = Attachment::detect(title);

                let location = match s3.bucket_exists(bucket.clone()).await {
                    Ok(v) => v,
                    Err(_) => s3.create_bucket(bucket.clone()).await?,
                };
                let mut buffer = Vec::new();

                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(Error::from)?;
                    buffer.extend_from_slice(&data);
                }
                let size = buffer.len();
                s3.put_object(bucket.clone(), name.clone(), buffer).await?;

                let url = match s3.endpoint {
                    Some(ref v) => format!("{}/{}/{}", v, bucket, name),
                    // https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region
                    None => format!(
                        "https://s3-{}.amazonaws.com/{}/{}",
                        location.unwrap_or_else(|| "".to_string()),
                        bucket,
                        name
                    ),
                };
                AttachmentDao::create(
                    db,
                    user.id,
                    title,
                    &content_type,
                    &url,
                    (size << 10) as i32,
                )?;
            }
        }
    }
    Ok(HttpResponse::Ok().finish())
}
