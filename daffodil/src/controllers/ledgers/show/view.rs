use std::collections::HashMap;

use askama::Template;
use camelia::{
    controllers::i18n as tpl_i18n,
    graphql::{attachment::Show as Attachment, site::InfoResponse as SiteInfo},
    models::{attachment::Dao as AttachmentDao, user::Details as UserDetails},
    orm::postgresql::Connection as Db,
};
use chrono::Duration;
use chrono::{Datelike, Days, Months, NaiveDate, NaiveDateTime, NaiveTime};
use hyper::StatusCode;
use log::debug;
use palm::{
    cache::redis::ClusterConnection as Cache, crypto::aes::Aes, iso4217::Currency as CurrencyItem,
    minio::Client as Minio, HttpError, Result,
};

use super::super::super::super::models::{
    bill::{Dao as BillDao, Item as BillItem},
    ledger::Item as LedgerItem,
};

#[derive(Template)]
#[template(path = "ledgers/show.html")]
pub struct Show {
    pub ledger: Ledger,
    pub site: SiteInfo,
    pub i18n: HashMap<String, String>,
}

impl Show {
    pub async fn new(
        db: &mut Db,
        ch: &mut Cache,
        s3: &Minio,
        aes: &Aes,
        lang: &str,
        item: &LedgerItem,
        ttl: Duration,
    ) -> Result<Self> {
        let it = Self {
            ledger: Ledger::new(db, s3, item, ttl).await?,
            site: SiteInfo::new(db, ch, aes, lang)?,
            i18n: tpl_i18n(db, lang)?,
        };
        Ok(it)
    }
}

pub struct Ledger {
    pub owner: UserDetails,
    pub name: String,
    pub summary: String,
    pub currencies: Vec<Currency>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Ledger {
    pub async fn new(db: &mut Db, s3: &Minio, it: &LedgerItem, ttl: Duration) -> Result<Self> {
        let mut currencies = Vec::new();
        for currency in BillDao::currencies(db, it.id)?.iter() {
            let it = Currency::new(db, s3, it.id, currency, ttl).await?;
            currencies.push(it);
        }
        let it = Self {
            owner: UserDetails::new(db, it.owner_id)?,
            name: it.name.clone(),
            summary: it.summary.clone(),
            currencies,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
            created_at: it.created_at,
        };
        Ok(it)
    }
}

pub struct Currency {
    pub inventory: Inventory,
    pub years: Vec<Year>,
}

impl Currency {
    pub async fn new(
        db: &mut Db,
        s3: &Minio,
        ledger: i32,
        currency: &str,
        ttl: Duration,
    ) -> Result<Self> {
        let from = BillDao::first_by_ledger_and_currency(db, ledger, currency)?.paid_at;
        let to = BillDao::latest_by_ledger_and_currency(db, ledger, currency)?.paid_at;

        debug!("select {currency} bills from {from} to {to}");
        let mut years = Vec::new();
        for year in from.year()..=to.year() {
            let it = Year::new(db, s3, ledger, currency, year, ttl).await?;
            if !it.months.is_empty() {
                years.push(it);
            }
        }

        Ok(Self {
            years,
            inventory: Inventory::new(db, ledger, currency, from, to)?,
        })
    }
}

pub struct Inventory {
    pub income: i64,
    pub expense: i64,
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
        let (income, expense) =
            BillDao::inventory_by_currency_ledger_and_dates(db, ledger, currency, from, to)?;
        let it = Self {
            income,
            expense,
            currency: CurrencyItem::new(currency)?,
        };
        Ok(it)
    }
}

pub struct Year {
    pub inventory: Inventory,
    pub year: i32,
    pub months: Vec<Month>,
}

impl Year {
    pub async fn new(
        db: &mut Db,
        s3: &Minio,
        ledger: i32,
        currency: &str,
        year: i32,
        ttl: Duration,
    ) -> Result<Self> {
        let from = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("bad year({year}")),
            )))?
            .and_time(NaiveTime::MIN);
        let to = from
            .checked_add_months(Months::new(12))
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("bad next year({year})")),
            )))?;
        debug!("select {currency} year bills from {from} to {to}");

        let mut months = Vec::new();
        {
            let mut cur = from;
            loop {
                let it = Month::new(db, s3, ledger, currency, year, cur.month(), ttl).await?;
                if !it.days.is_empty() {
                    months.push(it);
                }

                cur = cur
                    .checked_add_months(Months::new(1))
                    .ok_or(Box::new(HttpError(
                        StatusCode::BAD_REQUEST,
                        Some(format!("bad next month({from})")),
                    )))?;
                if cur >= to {
                    break;
                }
            }
        }

        Ok(Self {
            year,
            months,
            inventory: Inventory::new(db, ledger, currency, from, to)?,
        })
    }
}

pub struct Month {
    pub days: Vec<Day>,
    pub month: u32,
    pub inventory: Inventory,
}

impl Month {
    pub async fn new(
        db: &mut Db,
        s3: &Minio,
        ledger: i32,
        currency: &str,
        year: i32,
        month: u32,
        ttl: Duration,
    ) -> Result<Self> {
        let from = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("bad month({year},{month})")),
            )))?
            .and_time(NaiveTime::MIN);
        let to = from
            .checked_add_months(Months::new(1))
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("bad next month({year},{month})")),
            )))?;
        debug!("select {currency} month bills from {from} to {to}");

        let mut days = Vec::new();
        {
            let mut cur = from;
            loop {
                let it = Day::new(db, s3, ledger, currency, (year, month, cur.day()), ttl).await?;
                if !it.bills.is_empty() {
                    days.push(it);
                }

                cur = cur
                    .checked_add_days(Days::new(1))
                    .ok_or(Box::new(HttpError(
                        StatusCode::BAD_REQUEST,
                        Some(format!("bad next day({from})")),
                    )))?;
                if cur >= to {
                    break;
                }
            }
        }

        Ok(Self {
            days,
            month,
            inventory: Inventory::new(db, ledger, currency, from, to)?,
        })
    }
}

pub struct Day {
    pub bills: Vec<Bill>,
    pub day: u32,
    pub inventory: Inventory,
}

impl Day {
    pub async fn new(
        db: &mut Db,
        s3: &Minio,
        ledger: i32,
        currency: &str,
        (year, month, day): (i32, u32, u32),
        ttl: Duration,
    ) -> Result<Self> {
        let from = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("bad date({year},{month},{day})")),
            )))?
            .and_time(NaiveTime::MIN);
        let to = from
            .checked_add_days(Days::new(1))
            .ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(format!("bad next date({year},{month},{day})")),
            )))?;
        debug!("select {currency} day bills from {from} to {to}");

        let mut bills = Vec::new();
        for it in BillDao::by_currency_ledger_and_dates(db, ledger, currency, from, to)?.iter() {
            let it = Bill::new(db, s3, it, ttl).await?;
            bills.push(it);
        }
        Ok(Self {
            bills,
            day,
            inventory: Inventory::new(db, ledger, currency, from, to)?,
        })
    }
}

pub struct Bill {
    pub user: UserDetails,
    pub summary: String,
    pub amount: i32,
    pub currency: CurrencyItem,
    pub merchant: String,
    pub category: String,
    pub paid_at: NaiveDateTime,
    pub paid_by: String,
    pub attachments: Vec<Attachment>,
    pub updated_at: NaiveDateTime,
}

impl Bill {
    pub async fn new(db: &mut Db, s3: &Minio, it: &BillItem, ttl: Duration) -> Result<Self> {
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
            amount: it.amount,
            currency: CurrencyItem::new(&it.currency)?,
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
