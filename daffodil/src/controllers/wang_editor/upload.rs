use std::ops::Deref;

use actix_multipart::form::MultipartForm;
use actix_web::{
    error::ErrorInternalServerError, post, web, Error as WebError, Responder, Result as WebResult,
};
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, s3::Client as S3,
    session::Session,
};
use serde::Serialize;

use super::super::super::graphql::NAME;
use super::super::attachments::UploadForm;

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

#[post("/")]
pub async fn image(
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
    let it = form
        .execute(&ss, db, jwt, s3, NAME)
        .await
        .map_err(|e| -> WebError { ErrorInternalServerError(e) })?;
    Ok(web::Json(it))
}

#[post("/")]
pub async fn video(
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
    let it = form
        .execute(&ss, db, jwt, s3, NAME)
        .await
        .map_err(|e| -> WebError { ErrorInternalServerError(e) })?;
    Ok(web::Json(it))
}
