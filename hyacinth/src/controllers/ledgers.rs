use std::ops::{Deref, DerefMut};

use actix_web::{error::ErrorBadRequest, get, web, Responder, Result as WebResult};
use askama::Template;
use chrono::{Datelike, Duration, Months, NaiveDate, NaiveDateTime, NaiveTime};
use hyper::StatusCode;
use petunia::{
    jwt::{openssl::OpenSsl as Jwt, Jwt as JwtProvider},
    orm::postgresql::{Connection as Db, Pool as DbPool},
    try_web, HttpError, Result,
};

use super::super::{
    layout::bootstrap5::{Dropdown, Layout, Link, NavBar},
    models::{
        ledger::{Dao as LedgerDao, Item as Ledger},
        transaction::Dao as TransactionDao,
    },
};

#[derive(Template)]
#[template(path = "ledgers/show.html")]
struct Show {
    ledger: Ledger,
    layout: Layout,
}

impl Link {
    fn by_date(home: &str, it: NaiveDateTime) -> Self {
        Self {
            label: it.format("%Y-%m").to_string(),
            to: format!("{}by-month/{}-{}", home, it.year(), it.month()),
        }
    }
}

impl Dropdown {
    fn by_date_range(home: &str, begin: NaiveDateTime, end: NaiveDateTime) -> Vec<Self> {
        let mut items = Vec::new();
        items.push(Self::by_year_month(home, end, begin.year(), begin.month()));
        {
            let mut i = 1;
            loop {
                i += 1;
                let it = Self::by_year_month(home, end, begin.year() + i, 1);
                if it.items.is_empty() {
                    break;
                }
                items.push(it);
            }
        }
        items
    }

    fn by_year_month(home: &str, end: NaiveDateTime, year: i32, month: u32) -> Self {
        let mut items = Vec::new();

        if let Some(begin) = NaiveDate::from_ymd_opt(year, month, 1) {
            let begin = begin.and_time(NaiveTime::MIN);
            if begin < end {
                items.push(Link::by_date(home, begin));
            }
            if begin.month() < 12 {
                let mut i = 1;
                loop {
                    i += 1;
                    if let Some(it) = begin.checked_add_months(Months::new(i)) {
                        if it > end {
                            break;
                        }
                        items.push(Link::by_date(home, it));
                        if it.month() == 12 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Self {
            items,
            label: format!("{year}"),
        }
    }
}

impl NavBar {
    pub fn by_ledger(db: &mut Db, ledger: &Ledger, home: &str) -> Self {
        let begin = match TransactionDao::first_by_ledger(db, ledger.id) {
            Ok(it) => it.traded_at,
            Err(e) => {
                log::error!("{:?}", e);
                ledger.created_at
            }
        };
        let end = match TransactionDao::last_by_ledger(db, ledger.id) {
            Ok(it) => it.traded_at,
            Err(e) => {
                log::error!("{:?}", e);
                ledger.created_at
            }
        };

        Self {
            items: Dropdown::by_date_range(home, begin, end),
        }
    }
}

fn home_url(token: &str) -> String {
    format!("/accounting/ledgers/{token}/")
}
#[get("/{token}/")]
pub async fn show(
    (db, jwt): (web::Data<DbPool>, web::Data<Jwt>),
    params: web::Path<(String,)>,
) -> WebResult<impl Responder> {
    let (token,) = params.into_inner();
    let home = home_url(&token);
    let uid = try_web!(jwt.verify(&token, AUDIENCE))?;
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let ledger = try_web!(LedgerDao::by_uid(db, &uid))?;
    let body = try_web!(Show {
        layout: Layout {
            title: ledger.label.clone(),
            nav_bar: NavBar::by_ledger(db, &ledger, &home),
            home,
        },
        ledger,
    }
    .render())?;
    Ok(web::Html::new(body))
}

#[get("/{token}/by-dates/{b_year}-{b_month}-{b_day}-{e_year}-{e_month}-{e_day}")]
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
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = try_web!(render(db, jwt, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}
#[get("/{token}/{year}-{month}-{day}/daily")]
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
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = try_web!(render(db, jwt, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}
#[get("/{token}/{year}-{month}-{day}/weekly")]
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
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = try_web!(render(db, jwt, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}
#[get("/{token}/{year}-{month}-{day}/monthly")]
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
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = try_web!(render(db, jwt, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}
#[get("/{token}/{year}-{month}-{day}/yearly")]
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
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = try_web!(render(db, jwt, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}
#[get("/{token}/by-month/{year}-{month}")]
pub async fn by_year_month(
    (db, jwt): (web::Data<DbPool>, web::Data<Jwt>),
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
    let body = try_web!(render(db, jwt, &token, begin, end).await)?;
    Ok(web::Html::new(body))
}

#[get("/{token}/by-year/{year}")]
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
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let jwt = jwt.into_inner();
    let jwt = jwt.deref();
    let body = try_web!(render(db, jwt, &token, begin, end).await)?;
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
    log::debug!("ledger range {begin} {end}");
    let _uid = jwt.verify(token, AUDIENCE)?;
    // TODO
    Ok("TODO".to_string())
}

pub const AUDIENCE: &str = "ledger.show";
