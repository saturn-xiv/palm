use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use actix_web::{
    get, http::header::ContentType, web, HttpResponse, Responder, Result as WebResult,
};
use askama::Template;
use camelia::{
    controllers::i18n as tpl_i18n,
    graphql::{attachment::Show as Attachment, site::info::Response as SiteInfo},
    models::{
        attachment::{cover as get_cover, Dao as AttachmentDao},
        user::Details as UserDetails,
    },
    orm::postgresql::{Connection as Db, Pool as DbPool},
};
use chrono::{Datelike, Days, Duration, Months, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use hyper::StatusCode;
use log::debug;
use palm::{
    cache::redis::{ClusterConnection as Cache, Pool as CachePool},
    crypto::aes::Aes,
    duration_from_days,
    iso4217::Currency as CurrencyItem,
    jwt::{openssl::Jwt, Jwt as JwtProvider},
    minio::Client as Minio,
    try_web, HttpError, Result,
};

use super::super::super::super::{
    models::{
        bill::{Dao as BillDao, Item as BillItem},
        ledger::{Dao as LedgerDao, Item as LedgerItem},
    },
    NAME,
};

#[get("/{token}/")]
pub async fn index(
    db: web::Data<DbPool>,
    ch: web::Data<CachePool>,
    jwt: web::Data<Jwt>,
    aes: web::Data<Aes>,
    s3: web::Data<Minio>,
    token: web::Path<(String, String)>,
) -> WebResult<impl Responder> {
    let (lang, token) = token.into_inner();
    let (_, uid) = try_web!(jwt.verify(&token, NAME, LedgerItem::SHOW))?;
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let aes = aes.deref();
    let aes = aes.deref();
    let s3 = s3.deref();
    let s3 = s3.deref();

    let item = try_web!(LedgerDao::by_uid(db, &uid))?;
    let from = try_web!(BillDao::first_by_ledger(db, item.id))?.paid_at;
    let to = Utc::now().naive_utc();

    let tpl = {
        let mut it = try_web!(Show::new(db, ch, s3, aes, &lang, &item, (from, to)).await)?;
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
    jwt: web::Data<Jwt>,
    aes: web::Data<Aes>,
    s3: web::Data<Minio>,
    token: web::Path<(String, String, i32)>,
) -> WebResult<impl Responder> {
    let (lang, token, year) = token.into_inner();
    let (_, uid) = try_web!(jwt.verify(&token, NAME, LedgerItem::SHOW))?;
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let aes = aes.deref();
    let aes = aes.deref();
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

    let item = try_web!(LedgerDao::by_uid(db, &uid))?;
    let tpl = {
        let mut it = try_web!(Show::new(db, ch, s3, aes, &lang, &item, (from, to)).await)?;
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
    jwt: web::Data<Jwt>,
    aes: web::Data<Aes>,
    s3: web::Data<Minio>,
    token: web::Path<(String, String, i32, u32)>,
) -> WebResult<impl Responder> {
    let (lang, token, year, month) = token.into_inner();
    let (_, uid) = try_web!(jwt.verify(&token, NAME, LedgerItem::SHOW))?;
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let aes = aes.deref();
    let aes = aes.deref();
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

    let item = try_web!(LedgerDao::by_uid(db, &uid))?;
    let tpl = {
        let mut it = try_web!(Show::new(db, ch, s3, aes, &lang, &item, (from, to)).await)?;
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
    jwt: web::Data<Jwt>,
    aes: web::Data<Aes>,
    s3: web::Data<Minio>,
    token: web::Path<(String, String, i32, u32, u32)>,
) -> WebResult<impl Responder> {
    let (lang, token, year, month, day) = token.into_inner();
    let (_, uid) = try_web!(jwt.verify(&token, NAME, LedgerItem::SHOW))?;
    let mut db = try_web!(db.get())?;
    let db = db.deref_mut();
    let mut ch = try_web!(ch.get())?;
    let ch = ch.deref_mut();
    let aes = aes.deref();
    let aes = aes.deref();
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

    let item = try_web!(LedgerDao::by_uid(db, &uid))?;
    let tpl = {
        let mut it = try_web!(Show::new(db, ch, s3, aes, &lang, &item, (from, to)).await)?;
        try_web!(it.set_archives_by_day())?;
        it
    };
    let body = try_web!(tpl.render())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[derive(Template)]
#[template(path = "ledgers/show.html")]
pub struct Show {
    pub ledger: Ledger,
    pub site: SiteInfo,
    pub i18n: HashMap<String, String>,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub archives: Vec<String>,
}

impl Show {
    pub async fn new(
        db: &mut Db,
        ch: &mut Cache,
        s3: &Minio,
        aes: &Aes,
        lang: &str,
        item: &LedgerItem,
        (from, to): (NaiveDateTime, NaiveDateTime),
    ) -> Result<Self> {
        let it = Self {
            ledger: Ledger::new(db, s3, item, (from, to, duration_from_days(7)?)).await?,
            site: SiteInfo::new(db, ch, aes, lang)?,
            i18n: tpl_i18n(db, lang)?,
            from,
            to,
            archives: Vec::new(),
        };
        Ok(it)
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
pub struct Ledger {
    pub owner: UserDetails,
    pub name: String,
    pub summary: String,
    pub cover: Attachment,
    pub currencies: Vec<ByCurrency>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Ledger {
    pub async fn new(
        db: &mut Db,
        s3: &Minio,
        it: &LedgerItem,
        args: (NaiveDateTime, NaiveDateTime, Duration),
    ) -> Result<Self> {
        let mut currencies = Vec::new();
        for currency in BillDao::currencies(db, it.id)?.iter() {
            let it = ByCurrency::new(db, s3, it.id, currency, args).await?;
            currencies.push(it);
        }
        let cover = {
            let it = get_cover::<LedgerItem>(db, it.id)?;
            Attachment::new(s3, &it, args.2).await?
        };
        let it = Self {
            owner: UserDetails::new(db, it.owner_id)?,
            name: it.name.clone(),
            summary: it.summary.clone(),
            cover,
            currencies,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
            created_at: it.created_at,
        };
        Ok(it)
    }
}
pub struct ByCurrency {
    pub inventory: Inventory,
    pub bills: Vec<Bill>,
}

impl ByCurrency {
    pub async fn new(
        db: &mut Db,
        s3: &Minio,
        ledger: i32,
        currency: &str,
        (from, to, ttl): (NaiveDateTime, NaiveDateTime, Duration),
    ) -> Result<Self> {
        debug!("select {currency} bills from {from} to {to}");
        let mut bills = Vec::new();
        for it in BillDao::by_currency_ledger_and_dates(db, ledger, currency, from, to)?.iter() {
            let it = Bill::new(db, s3, it, ttl).await?;
            bills.push(it);
        }

        Ok(Self {
            bills,
            inventory: Inventory::new(db, ledger, currency, from, to)?,
        })
    }
}

pub struct Inventory {
    pub income: String,
    pub expense: String,
    pub balance: String,
    pub currency: CurrencyItem,
}

impl Inventory {
    pub fn new(
        db: &mut Db,
        ledger: i32,
        currency: &str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<Self> {
        let currency = CurrencyItem::new(currency)?;
        let (income, expense, balance) =
            BillDao::inventory_by_currency_ledger_and_dates(db, ledger, &currency.code, from, to)?;
        let it = Self {
            income: Self::i2f(income, currency.unit),
            expense: Self::i2f(expense, currency.unit),
            balance: Self::i2f(balance, currency.unit),
            currency,
        };
        Ok(it)
    }
    fn i2f(v: i64, u: i32) -> String {
        format!("{}", (v as f32) / (10_i32.pow(u as u32) as f32))
    }
}

pub struct Bill {
    pub user: UserDetails,
    pub summary: String,
    pub amount: String,
    pub merchant: String,
    pub category: String,
    pub paid_at: NaiveDateTime,
    pub paid_by: String,
    pub attachments: Vec<Attachment>,
    pub updated_at: NaiveDateTime,
}

impl Bill {
    pub async fn new(db: &mut Db, s3: &Minio, it: &BillItem, ttl: Duration) -> Result<Self> {
        let amount = {
            let c = CurrencyItem::new(&it.currency)?;
            Inventory::i2f(it.amount as i64, c.unit)
        };
        let attachments = {
            let mut items = Vec::new();
            for it in AttachmentDao::by_resource::<BillItem>(db, it.id)?.iter() {
                let it = Attachment::new(s3, it, ttl).await?;
                items.push(it);
            }
            items
        };
        let it = Self {
            user: UserDetails::new(db, it.user_id)?,
            summary: it.summary.clone(),
            amount,
            merchant: it.merchant.clone(),
            category: it.category.clone(),
            paid_at: it.paid_at,
            paid_by: it.paid_by.clone(),
            attachments,
            updated_at: it.updated_at,
        };
        Ok(it)
    }
}
