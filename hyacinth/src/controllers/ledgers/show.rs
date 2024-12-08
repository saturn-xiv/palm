use std::ops::DerefMut;

use actix_web::{get, web, Responder, Result as WebResult};
use askama::Template;
use chrono::{Datelike, Months, NaiveDate, NaiveDateTime, NaiveTime};
use petunia::{
    jwt::{openssl::OpenSsl as Jwt, Jwt as JwtProvider},
    orm::postgresql::{Connection as Db, Pool as DbPool},
    try_web,
};

use super::super::super::{
    layout::bootstrap5::{Dropdown, Layout, Link, NavBar},
    models::{
        ledger::{Dao as LedgerDao, Item as Ledger},
        transaction::Dao as TransactionDao,
    },
};
use super::{home_url, AUDIENCE};

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

#[get("/{token}/")]
pub async fn get(
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
