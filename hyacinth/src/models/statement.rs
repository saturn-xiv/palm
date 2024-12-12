use chrono::NaiveDateTime;
use chrono_tz::Tz;
use diesel::{insert_into, prelude::*, result::Error::NotFound};
use hyper::StatusCode;
use petunia::{orm::postgresql::Connection, HttpError, Result};
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

use super::super::schema::bookkeeper_statements;

#[derive(EnumDisplay, EnumString, Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    #[default]
    Credit,
    Debit,
}

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub account_id: i32,
    pub transaction_id: i32,
    pub entry_id: i32,
    pub currency_id: i32,
    pub amount: i32,
    pub r#type: String,
    pub opening_balance: i32,
    pub closing_balance: i32,
    pub traded_at: NaiveDateTime,
    pub timezone: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn create(
        &mut self,
        transaction: (i32, i32, i32, i32),
        amount: (i32, i32, Type),
        balances: (i32, i32),
        traded: (NaiveDateTime, Tz),
    ) -> Result<()>;
    fn latest(&mut self, account: i32) -> Result<Option<Item>>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_transaction(&mut self, transaction: i32) -> Result<Vec<Item>>;
    fn count_by_ledger(
        &mut self,
        ledger: i32,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<i64>;
    fn by_ledger(
        &mut self,
        ledger: i32,
        from: NaiveDateTime,
        to: NaiveDateTime,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        (ledger, account, transaction, entry): (i32, i32, i32, i32),
        (currency, amount, type_): (i32, i32, Type),
        (opening_balance, closing_balance): (i32, i32),
        (traded_at, timezone): (NaiveDateTime, Tz),
    ) -> Result<()> {
        insert_into(bookkeeper_statements::dsl::bookkeeper_statements)
            .values((
                bookkeeper_statements::dsl::ledger_id.eq(ledger),
                bookkeeper_statements::dsl::account_id.eq(account),
                bookkeeper_statements::dsl::transaction_id.eq(transaction),
                bookkeeper_statements::dsl::entry_id.eq(entry),
                bookkeeper_statements::dsl::currency_id.eq(currency),
                bookkeeper_statements::dsl::amount.eq(amount),
                bookkeeper_statements::dsl::type_.eq(&type_.to_string()),
                bookkeeper_statements::dsl::opening_balance.eq(opening_balance),
                bookkeeper_statements::dsl::closing_balance.eq(closing_balance),
                bookkeeper_statements::dsl::traded_at.eq(traded_at),
                bookkeeper_statements::dsl::timezone.eq(&timezone.to_string()),
            ))
            .execute(self)?;
        Ok(())
    }

    fn latest(&mut self, account: i32) -> Result<Option<Item>> {
        match bookkeeper_statements::dsl::bookkeeper_statements
            .filter(bookkeeper_statements::dsl::account_id.eq(account))
            .order(bookkeeper_statements::dsl::created_at.desc())
            .first(self)
        {
            Ok(it) => Ok(Some(it)),
            Err(NotFound) => Ok(None),
            Err(e) => Err(Box::new(HttpError(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(e.to_string()),
            ))),
        }
    }
    fn by_id(&mut self, id: i32) -> Result<Item> {
        let it = bookkeeper_statements::dsl::bookkeeper_statements
            .filter(bookkeeper_statements::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn by_transaction(&mut self, transaction: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_statements::dsl::bookkeeper_statements
            .filter(bookkeeper_statements::dsl::transaction_id.eq(transaction))
            .order(bookkeeper_statements::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn count_by_ledger(
        &mut self,
        ledger: i32,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<i64> {
        let cnt: i64 = bookkeeper_statements::dsl::bookkeeper_statements
            .count()
            .filter(bookkeeper_statements::dsl::ledger_id.eq(ledger))
            .filter(bookkeeper_statements::dsl::created_at.ge(from))
            .filter(bookkeeper_statements::dsl::created_at.lt(to))
            .get_result(self)?;
        Ok(cnt)
    }
    fn by_ledger(
        &mut self,
        ledger: i32,
        from: NaiveDateTime,
        to: NaiveDateTime,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Item>> {
        let items = bookkeeper_statements::dsl::bookkeeper_statements
            .filter(bookkeeper_statements::dsl::ledger_id.eq(ledger))
            .order(bookkeeper_statements::dsl::created_at.desc())
            .filter(bookkeeper_statements::dsl::created_at.ge(from))
            .filter(bookkeeper_statements::dsl::created_at.lt(to))
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
}
