use std::ops::{Deref, DerefMut};

use actix_web::{
    get, http::header::ContentType, web, HttpResponse, Responder, Result as WebResult,
};
use askama::Template;
use camelia::{
    controllers::{Jasmine, Loquat},
    orm::postgresql::{Connection as Db, Pool as DbPool},
};
use chrono::{Datelike, Days, Months, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use hibiscus::cache::redis::{ClusterConnection as Cache, Pool as CachePool};
use hyper::StatusCode;
use palm::{jasmine::S3, try_web, HttpError, Jwt, Result, Secret};

use super::super::super::super::{
    models::{
        book::{Dao as BookDao, Item as BookItem},
        transaction::Dao as TransactionDao,
    },
    NAME,
};

const AUDIENCE: &str = "books.show";

#[get("/{token}/")]
pub async fn index(
    db: web::Data<DbPool>,
    ch: web::Data<CachePool>,
    jwt_aes: web::Data<Loquat>,
    s3: web::Data<Jasmine>,
    params: web::Path<(String, String)>,
) -> WebResult<impl Responder> {
    let (lang, token) = params.into_inner();
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let jwt_aes = jwt_aes.deref();
    let jwt_aes = jwt_aes.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();

    let (_, uid, _) = try_web!(Jwt::verify::<String>(jwt_aes, &token, NAME, AUDIENCE))?;
    let item = try_web!(BookDao::by_uid(db, &uid))?;
    let from = try_web!(TransactionDao::first_by_book(db, item.id))?.paid_at;
    let to = Utc::now().naive_utc();

    let tpl = {
        let mut it = try_web!(Show::new(db, ch, s3, jwt_aes, &lang, &item, (from, to)).await)?;
        it.set_archives();
        it
    };
    let body = try_web!(tpl.render())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[get("/{token}/{year}")]
pub async fn by_year(
    db: web::Data<DbPool>,
    ch: web::Data<CachePool>,
    jwt_aes: web::Data<Loquat>,
    s3: web::Data<Jasmine>,
    token: web::Path<(String, String, i32)>,
) -> WebResult<impl Responder> {
    let (lang, token, year) = token.into_inner();
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let jwt_aes = jwt_aes.deref();
    let jwt_aes = jwt_aes.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();

    let from = try_web!(
        NaiveDate::from_ymd_opt(year, 1, 1).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some(format!("bad year({year}")),
        )))
    )?
    .and_time(NaiveTime::MIN);
    let to = try_web!(from
        .checked_add_months(Months::new(12))
        .ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some(format!("bad next year({year})")),
        ))))?;

    let (_, uid, _) = try_web!(Jwt::verify::<String>(jwt_aes, &token, NAME, AUDIENCE))?;
    let item = try_web!(BookDao::by_uid(db, &uid))?;
    let tpl = {
        let mut it = try_web!(Show::new(db, ch, s3, jwt_aes, &lang, &item, (from, to)).await)?;
        it.set_archives_by_year();
        it
    };
    let body = try_web!(tpl.render())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[get("/{token}/{year}-{month}")]
pub async fn by_month(
    db: web::Data<DbPool>,
    ch: web::Data<CachePool>,
    jwt_aes: web::Data<Loquat>,
    s3: web::Data<Jasmine>,
    token: web::Path<(String, String, i32, u32)>,
) -> WebResult<impl Responder> {
    let (lang, token, year, month) = token.into_inner();
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let jwt_aes = jwt_aes.deref();
    let jwt_aes = jwt_aes.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();

    let from = try_web!(
        NaiveDate::from_ymd_opt(year, month, 1).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some(format!("bad month({year},{month})")),
        )))
    )?
    .and_time(NaiveTime::MIN);
    let to = try_web!(from
        .checked_add_months(Months::new(1))
        .ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some(format!("bad next month({year},{month})")),
        ))))?;

    let (_, uid, _) = try_web!(Jwt::verify::<String>(jwt_aes, &token, NAME, AUDIENCE))?;
    let item = try_web!(BookDao::by_uid(db, &uid))?;
    let tpl = {
        let mut it = try_web!(Show::new(db, ch, s3, jwt_aes, &lang, &item, (from, to)).await)?;
        try_web!(it.set_archives_by_month())?;
        it
    };
    let body = try_web!(tpl.render())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[get("/{token}/{year}-{month}-{day}")]
pub async fn by_day(
    db: web::Data<DbPool>,
    ch: web::Data<CachePool>,
    jwt_aes: web::Data<Loquat>,
    s3: web::Data<Jasmine>,
    token: web::Path<(String, String, i32, u32, u32)>,
) -> WebResult<impl Responder> {
    let (lang, token, year, month, day) = token.into_inner();
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let jwt_aes = jwt_aes.deref();
    let jwt_aes = jwt_aes.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();

    let from = try_web!(
        NaiveDate::from_ymd_opt(year, month, day).ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some(format!("bad date({year},{month},{day})")),
        )))
    )?
    .and_time(NaiveTime::MIN);
    let to = try_web!(from
        .checked_add_days(Days::new(1))
        .ok_or(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some(format!("bad next date({year},{month},{day})")),
        ))))?;

    let (_, uid, _) = try_web!(Jwt::verify::<String>(jwt_aes, &token, NAME, AUDIENCE))?;
    let item = try_web!(BookDao::by_uid(db, &uid))?;
    let tpl = {
        let mut it = try_web!(Show::new(db, ch, s3, jwt_aes, &lang, &item, (from, to)).await)?;
        try_web!(it.set_archives_by_day())?;
        it
    };
    let body = try_web!(tpl.render())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[derive(Template)]
#[template(path = "books/show.html")]
pub struct Show {
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub archives: Vec<String>,
}

impl Show {
    pub async fn new<S: S3, A: Secret>(
        _db: &mut Db,
        _ch: &mut Cache,
        _s3: &S,
        _aes: &A,
        _lang: &str,
        _item: &BookItem,
        (_from, _to): (NaiveDateTime, NaiveDateTime),
    ) -> Result<Self> {
        // FIXME
        todo!()
    }
    fn set_archives(&mut self) {
        for year in self.from.year()..=self.to.year() {
            self.archives.push(format!("{year}"));
        }
    }
    fn set_archives_by_year(&mut self) {
        let year = self.from.year();
        for month in 1..=12 {
            self.archives.push(format!("{year}-{month:0>2}"));
        }
    }
    fn set_archives_by_month(&mut self) -> Result<()> {
        let year = self.from.year();
        let month = self.from.month();
        for day in 1..=31 {
            self.archives.push(format!("{year}-{month:0>2}-{day:0>2}"));
            let tomorrow =
                self.from
                    .checked_add_days(Days::new(day))
                    .ok_or(Box::new(HttpError(
                        StatusCode::BAD_REQUEST,
                        Some(format!("bad date({year},{month},{day})")),
                    )))?;
            if tomorrow >= self.to {
                break;
            }
        }

        Ok(())
    }
    fn set_archives_by_day(&mut self) -> Result<()> {
        let gap: usize = 7;

        for it in self
            .from
            .checked_add_days(Days::new(gap as u64))
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("bad two weeks before({})", self.from)),
            )))?
            .date()
            .iter_days()
            .rev()
            .take(gap * 2 + 1)
        {
            let it = it.and_time(NaiveTime::MIN);
            let year = it.year();
            let month = it.month();
            let day = it.day();
            self.archives.push(format!("{year}-{month:0>2}-{day:0>2}"));
        }
        Ok(())
    }
}
