pub mod tpl;

use std::ops::{Deref, DerefMut};

use actix_web::{error::ErrorBadRequest, get, web, Responder, Result as WebResult};
use askama::Template;
use chrono::{Datelike, Duration, Months, NaiveDate, NaiveDateTime, NaiveTime};
use hyper::StatusCode;
use petunia::{
    jwt::{openssl::OpenSsl as Jwt, Jwt as JwtProvider},
    orm::postgresql::{Connection as Db, Pool as DbPool},
    s3::Client as S3,
    session::Session,
    try_web, HttpError, Result,
};

use super::super::models::ledger::Dao as LedgerDao;

pub const AUDIENCE: &str = "bookkeeper.statement.show";

pub fn home_url(token: &str) -> String {
    format!("/accounting/statements/{token}/")
}

#[get("/{token}/by-dates/{b_year}-{b_month}-{b_day}-{e_year}-{e_month}-{e_day}")]
pub async fn by_date_range(
    (ss, db, jwt, s3): (Session, web::Data<DbPool>, web::Data<Jwt>, web::Data<S3>),
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
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();
    let body = try_web!(render(db, s3, jwt, &ss.lang, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}
#[get("/{token}/{year}-{month}-{day}/daily")]
pub async fn daily_by_date(
    (ss, db, jwt, s3): (Session, web::Data<DbPool>, web::Data<Jwt>, web::Data<S3>),
    params: web::Path<(String, i32, u32, u32)>,
) -> WebResult<impl Responder> {
    let (token, year, month, day) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin + Duration::days(1);

    let db = db.into_inner();
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();
    let body = try_web!(render(db, s3, jwt, &ss.lang, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}
#[get("/{token}/{year}-{month}-{day}/weekly")]
pub async fn weekly_by_date(
    (ss, db, jwt, s3): (Session, web::Data<DbPool>, web::Data<Jwt>, web::Data<S3>),
    params: web::Path<(String, i32, u32, u32)>,
) -> WebResult<impl Responder> {
    let (token, year, month, day) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin + Duration::weeks(1);

    let db = db.into_inner();
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();
    let body = try_web!(render(db, s3, jwt, &ss.lang, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}
#[get("/{token}/{year}-{month}-{day}/monthly")]
pub async fn monthly_by_date(
    (ss, db, jwt, s3): (Session, web::Data<DbPool>, web::Data<Jwt>, web::Data<S3>),
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
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();
    let body = try_web!(render(db, s3, jwt, &ss.lang, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}
#[get("/{token}/{year}-{month}-{day}/yearly")]
pub async fn yearly_by_date(
    (ss, db, jwt, s3): (Session, web::Data<DbPool>, web::Data<Jwt>, web::Data<S3>),
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
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();
    let body = try_web!(render(db, s3, jwt, &ss.lang, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}
#[get("/{token}/by-month/{year}-{month}")]
pub async fn by_year_month(
    (ss, db, jwt, s3): (Session, web::Data<DbPool>, web::Data<Jwt>, web::Data<S3>),
    params: web::Path<(String, i32, u32)>,
) -> WebResult<impl Responder> {
    let (token, year, month) = params.into_inner();
    let begin = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or(ErrorBadRequest("bad year month day"))?
        .and_time(NaiveTime::MIN);
    let end = begin
        .checked_add_months(Months::new(1))
        .ok_or(ErrorBadRequest("bad year month day"))?;

    let db = db.into_inner();
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();
    let body = try_web!(render(db, s3, jwt, &ss.lang, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}

#[get("/{token}/by-year/{year}")]
pub async fn by_year(
    (ss, db, jwt, s3): (Session, web::Data<DbPool>, web::Data<Jwt>, web::Data<S3>),
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
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();
    let body = try_web!(render(db, s3, jwt, &ss.lang, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}

async fn render(
    db: &mut Db,
    s3: &S3,
    jwt: &Jwt,
    lang: &str,
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
    log::debug!("ledger range {begin} {end}");
    let uid = jwt.verify(token, AUDIENCE)?;
    let home = home_url(token);
    let ledger = LedgerDao::by_uid(db, &uid)?;
    let body = {
        let it = tpl::Index::new(
            db,
            s3,
            &ledger,
            lang,
            &home,
            (begin, end, Some(Duration::hours(1))),
        )
        .await?;
        it.render()?
    };

    Ok(body)
}
