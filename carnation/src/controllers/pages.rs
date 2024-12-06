use std::ops::{Deref, DerefMut};

use actix_multipart::form::MultipartForm;
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get, post, web, Responder, Result as WebResult,
};

use casbin::Enforcer;
use chrono::{Datelike, Days, Months, NaiveDate, NaiveDateTime, NaiveTime};
use daffodil::{controllers::attachments::UploadForm, session::current_user};
use hyper::StatusCode;
use petunia::{
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    s3::Client as S3,
    session::Session,
    try_web, HttpError, Result,
};
use tokio::sync::Mutex;

use crate::graphql::ROLE_MANAGER;

use super::super::{models::page::Item as Page, NAME};

#[post("/upload")]
pub async fn upload(
    ss: Session,
    db: web::Data<DbPool>,
    jwt: web::Data<Jwt>,
    s3: web::Data<S3>,
    enforcer: web::Data<Mutex<Enforcer>>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> WebResult<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let jwt = jwt.deref();
    let s3 = s3.deref();

    {
        let mut db = try_web!(db.get())?;
        let db = db.deref_mut();
        let (_, user) = try_web!(current_user(&ss, db, jwt))?;
        {
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            try_web!(user.has(enf, ROLE_MANAGER))?;
        }
    }

    let it = try_web!(form.save::<Page>(&ss, db, jwt, s3, NAME).await)?;
    Ok(web::Json(it))
}

#[get("/latest")]
pub async fn latest(db: web::Data<DbPool>) -> WebResult<impl Responder> {
    let db = db.into_inner();
    let mut db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;

    let body = render_index(&mut db)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}

#[get("/{year}-{month}-{day}")]
pub async fn by_day(
    db: web::Data<DbPool>,
    params: web::Path<(i32, u32, u32)>,
) -> WebResult<impl Responder> {
    let (year, month, day) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin
        .checked_add_days(Days::new(1))
        .ok_or(ErrorBadRequest("bad plug one day"))?;

    let db = db.into_inner();
    let mut db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;

    let body = render(&mut db, begin, end)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}

#[get("/{year}-{month}")]
pub async fn by_month(
    db: web::Data<DbPool>,
    params: web::Path<(i32, u32)>,
) -> WebResult<impl Responder> {
    let (year, month) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin
        .checked_add_months(Months::new(1))
        .ok_or(ErrorBadRequest("bad plug one month"))?;

    let db = db.into_inner();
    let mut db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;

    let body = render(&mut db, begin, end)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}

#[get("/{year}")]
pub async fn by_year(
    db: web::Data<DbPool>,
    params: web::Path<(i32,)>,
) -> WebResult<impl Responder> {
    let (year,) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, 1, 1)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin
        .with_year(year + 1)
        .ok_or(ErrorBadRequest("bad plus one year"))?;

    let db = db.into_inner();
    let mut db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;

    let body = render(&mut db, begin, end)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}

async fn render(_db: &mut Db, begin: NaiveDateTime, end: NaiveDateTime) -> Result<String> {
    if begin >= end {
        return Err(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("end-time should after the begin-time".to_string()),
        )));
    }
    // TODO
    todo!()
}

async fn render_index(_db: &mut Db) -> Result<String> {
    // TODO
    todo!()
}
