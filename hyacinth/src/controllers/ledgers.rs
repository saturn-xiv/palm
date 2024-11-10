use std::ops::{Deref, DerefMut};

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get, web, Responder, Result as WebResult,
};
use chrono::{Datelike, Duration, Months, NaiveDate, NaiveDateTime, NaiveTime};
use hyper::StatusCode;
use petunia::{
    jwt::{openssl::OpenSsl as Jwt, Jwt as JwtProvider},
    orm::postgresql::{Connection as Db, Pool as DbPool},
    HttpError, Result,
};

#[get("/{uid}/{b_year}-{b_month}-{b_day}-{e_year}-{e_month}-{e_day}")]
pub async fn by_date_range(
    (db, jwt): (web::Data<DbPool>, web::Data<Jwt>),
    params: web::Path<(String, i32, u32, u32, i32, u32, u32)>,
) -> WebResult<impl Responder> {
    let (token, b_year, b_month, b_day, e_year, e_month, e_day) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(b_year, b_month, b_day)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = NaiveDate::from_ymd_opt(e_year, e_month, e_day)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);

    let db = db.into_inner();
    let mut db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = render(db, jwt, &token, begin, end)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}
#[get("/{uid}/{year}-{month}-{day}/daily")]
pub async fn daily_by_date(
    (db, jwt): (web::Data<DbPool>, web::Data<Jwt>),
    params: web::Path<(String, i32, u32, u32)>,
) -> WebResult<impl Responder> {
    let (token, year, month, day) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin + Duration::days(1);

    let db = db.into_inner();
    let mut db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = render(db, jwt, &token, begin, end)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}
#[get("/{uid}/{year}-{month}-{day}/weekly")]
pub async fn weekly_by_date(
    (db, jwt): (web::Data<DbPool>, web::Data<Jwt>),
    params: web::Path<(String, i32, u32, u32)>,
) -> WebResult<impl Responder> {
    let (token, year, month, day) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin + Duration::weeks(1);

    let db = db.into_inner();
    let mut db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = render(db, jwt, &token, begin, end)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}
#[get("/{uid}/{year}-{month}-{day}/monthly")]
pub async fn monthly_by_date(
    (db, jwt): (web::Data<DbPool>, web::Data<Jwt>),
    params: web::Path<(String, i32, u32, u32)>,
) -> WebResult<impl Responder> {
    let (token, year, month, day) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin
        .checked_add_months(Months::new(1))
        .ok_or(ErrorBadRequest("bad year month day"))?;

    let db = db.into_inner();
    let mut db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = render(db, jwt, &token, begin, end)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}
#[get("/{uid}/{year}-{month}-{day}/yearly")]
pub async fn yearly_by_date(
    (db, jwt): (web::Data<DbPool>, web::Data<Jwt>),
    params: web::Path<(String, i32, u32, u32)>,
) -> WebResult<impl Responder> {
    let (token, year, month, day) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin
        .checked_add_months(Months::new(1))
        .ok_or(ErrorBadRequest("bad year month day"))?;

    let db = db.into_inner();
    let mut db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = render(db, jwt, &token, begin, end)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}
#[get("/{uid}/{year}-{month}")]
pub async fn by_year_month(
    (db, jwt): (web::Data<DbPool>, web::Data<Jwt>),
    params: web::Path<(String, i32, u32)>,
) -> WebResult<impl Responder> {
    let (token, year, month) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin
        .with_year(year + 1)
        .ok_or(ErrorBadRequest("bad next year"))?;

    let db = db.into_inner();
    let mut db = db
        .get()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = render(db, jwt, &token, begin, end)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}

#[get("/{uid}/{year}")]
pub async fn by_year(
    (db, jwt): (web::Data<DbPool>, web::Data<Jwt>),
    params: web::Path<(String, i32)>,
) -> WebResult<impl Responder> {
    let (token, year) = params.into_inner();
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
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = render(db, jwt, &token, begin, end)
        .await
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    Ok(web::Html::new(body))
}

async fn render(
    _db: &mut Db,
    jwt: &Jwt,
    token: &str,
    begin: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<String> {
    if begin >= end {
        return Err(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("end-time should after the begin-time".to_string()),
        )));
    }
    let _uid = jwt.verify(token, AUDIENCE)?;
    // TODO
    todo!()
}

pub const AUDIENCE: &str = "ledger.show";
