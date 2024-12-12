use chrono::{Days, NaiveDateTime, NaiveTime, Utc};
use chrono_tz::Tz;
use diesel::{insert_into, prelude::*, update};
use hyper::StatusCode;
use petunia::{orm::postgresql::Connection, HttpError, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

use super::super::schema::bookkeeper_entries;

#[derive(EnumDisplay, EnumString, Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    #[default]
    Pending,
    Audited,
}

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub sn: String,
    pub transaction_id: i32,
    pub from_account_id: i32,
    pub to_account_id: i32,
    pub category_id: i32,
    pub merchant_id: i32,
    pub current_id: i32,
    pub amount: i32,
    pub memo: String,
    pub traded_at: NaiveDateTime,
    pub timezone: String,
    pub status: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Item {
    fn next_sn(db: &mut Connection, ledger: i32) -> Result<String> {
        let today = Utc::now().naive_utc().date();
        let nbf = today.and_time(NaiveTime::MIN);
        if let Some(exp) = nbf.checked_add_days(Days::new(1)) {
            let cnt: i64 = bookkeeper_entries::dsl::bookkeeper_entries
                .count()
                .filter(bookkeeper_entries::dsl::ledger_id.eq(ledger))
                .filter(bookkeeper_entries::dsl::created_at.ge(nbf))
                .filter(bookkeeper_entries::dsl::created_at.lt(exp))
                .get_result(db)?;
            return Ok(format!(
                "{}{:0>8X}",
                today.format("%Y%m%d"),
                (cnt + 1) as i32
            ));
        }
        Err(Box::new(HttpError(StatusCode::INTERNAL_SERVER_ERROR, None)))
    }
}

pub trait Dao {
    fn create(
        &mut self,
        bill: (i32, i32, i32),
        accounts: (i32, i32),
        merchant: (i32, i32, i32, &str),
        traded_at: (NaiveDateTime, Tz),
    ) -> Result<()>;
    fn update(
        &mut self,
        id: i32,
        category: i32,
        accounts: (i32, i32),
        merchant: (i32, i32, i32, &str),
        traded_at: (NaiveDateTime, Tz),
    ) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_transaction(&mut self, transaction: i32) -> Result<Vec<Item>>;
    fn count_by_ledger(&mut self, ledger: i32) -> Result<i64>;
    fn by_ledger(&mut self, ledger: i32, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn by_account(&mut self, account: i32) -> Result<Vec<Item>>;
    fn by_from_account(&mut self, account: i32) -> Result<Vec<Item>>;
    fn by_to_account(&mut self, account: i32) -> Result<Vec<Item>>;
    fn by_merchant(&mut self, merchant: i32) -> Result<Vec<Item>>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
    fn set_status(&mut self, id: i32, status: Status) -> Result<()>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        (ledger, transaction, category): (i32, i32, i32),
        (from_account, to_account): (i32, i32),
        (merchant, currency, amount, memo): (i32, i32, i32, &str),
        (traded_at, timezone): (NaiveDateTime, Tz),
    ) -> Result<()> {
        let sn = Item::next_sn(self, ledger)?;
        let now = Utc::now().naive_utc();
        insert_into(bookkeeper_entries::dsl::bookkeeper_entries)
            .values((
                bookkeeper_entries::dsl::ledger_id.eq(ledger),
                bookkeeper_entries::dsl::sn.eq(&sn),
                bookkeeper_entries::dsl::transaction_id.eq(transaction),
                bookkeeper_entries::dsl::category_id.eq(category),
                bookkeeper_entries::dsl::from_account_id.eq(from_account),
                bookkeeper_entries::dsl::to_account_id.eq(to_account),
                bookkeeper_entries::dsl::merchant_id.eq(merchant),
                bookkeeper_entries::dsl::currency_id.eq(currency),
                bookkeeper_entries::dsl::amount.eq(amount),
                bookkeeper_entries::dsl::memo.eq(memo),
                bookkeeper_entries::dsl::traded_at.eq(traded_at),
                bookkeeper_entries::dsl::timezone.eq(&timezone.to_string()),
                bookkeeper_entries::dsl::status.eq(&Status::Pending.to_string()),
                bookkeeper_entries::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(
        &mut self,
        id: i32,
        category: i32,
        (from_account, to_account): (i32, i32),
        (merchant, currency, amount, memo): (i32, i32, i32, &str),
        (traded_at, timezone): (NaiveDateTime, Tz),
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            bookkeeper_entries::dsl::bookkeeper_entries.filter(bookkeeper_entries::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_entries::dsl::category_id.eq(category),
                bookkeeper_entries::dsl::from_account_id.eq(from_account),
                bookkeeper_entries::dsl::to_account_id.eq(to_account),
                bookkeeper_entries::dsl::merchant_id.eq(merchant),
                bookkeeper_entries::dsl::currency_id.eq(currency),
                bookkeeper_entries::dsl::amount.eq(amount),
                bookkeeper_entries::dsl::memo.eq(memo),
                bookkeeper_entries::dsl::traded_at.eq(traded_at),
                bookkeeper_entries::dsl::timezone.eq(&timezone.to_string()),
                bookkeeper_entries::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bookkeeper_entries::dsl::bookkeeper_entries
            .filter(bookkeeper_entries::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn by_transaction(&mut self, transaction: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_entries::dsl::bookkeeper_entries
            .filter(bookkeeper_entries::dsl::transaction_id.eq(transaction))
            .order(bookkeeper_entries::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn count_by_ledger(&mut self, ledger: i32) -> Result<i64> {
        let cnt: i64 = bookkeeper_entries::dsl::bookkeeper_entries
            .count()
            .filter(bookkeeper_entries::dsl::ledger_id.eq(ledger))
            .get_result(self)?;
        Ok(cnt)
    }
    fn by_ledger(&mut self, ledger: i32, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = bookkeeper_entries::dsl::bookkeeper_entries
            .filter(bookkeeper_entries::dsl::ledger_id.eq(ledger))
            .order(bookkeeper_entries::dsl::updated_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_account(&mut self, account: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_entries::dsl::bookkeeper_entries
            .filter(
                bookkeeper_entries::dsl::from_account_id
                    .eq(account)
                    .or(bookkeeper_entries::dsl::to_account_id.eq(account)),
            )
            .order(bookkeeper_entries::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_from_account(&mut self, account: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_entries::dsl::bookkeeper_entries
            .filter(bookkeeper_entries::dsl::from_account_id.eq(account))
            .order(bookkeeper_entries::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_to_account(&mut self, account: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_entries::dsl::bookkeeper_entries
            .filter(bookkeeper_entries::dsl::to_account_id.eq(account))
            .order(bookkeeper_entries::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_merchant(&mut self, merchant: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_entries::dsl::bookkeeper_entries
            .filter(bookkeeper_entries::dsl::merchant_id.eq(merchant))
            .order(bookkeeper_entries::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            bookkeeper_entries::dsl::bookkeeper_entries.filter(bookkeeper_entries::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_entries::dsl::deleted_at.eq(&Some(now)),
                bookkeeper_entries::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            bookkeeper_entries::dsl::bookkeeper_entries.filter(bookkeeper_entries::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_entries::dsl::deleted_at.eq(&None::<NaiveDateTime>),
                bookkeeper_entries::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_status(&mut self, id: i32, status: Status) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            bookkeeper_entries::dsl::bookkeeper_entries.filter(bookkeeper_entries::dsl::id.eq(id));
        update(it)
            .set((
                bookkeeper_entries::dsl::status.eq(&status.to_string()),
                bookkeeper_entries::dsl::updated_at.eq(now),
            ))
            .execute(self)?;
        Ok(())
    }
}
