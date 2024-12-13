use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*, result::Error::NotFound};
use hyper::StatusCode;
use petunia::{orm::postgresql::Connection, HttpError, Result};
use serde::{Deserialize, Serialize};

use super::super::schema::bookkeeper_statements;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub transaction_id: i32,
    pub transaction_memo: String,
    pub entry_id: i32,
    pub entry_memo: String,
    pub entry_sn: String,
    pub category_id: i32,
    pub category_label: String,
    pub merchant_id: i32,
    pub merchant_label: String,
    pub debtor_id: i32,
    pub debtor_label: String,
    pub debtor_opening_balance: i32,
    pub debtor_closing_balance: i32,
    pub creditor_id: i32,
    pub creditor_label: String,
    pub creditor_opening_balance: i32,
    pub creditor_closing_balance: i32,
    pub currency_id: i32,
    pub currency_code: String,
    pub currency_name: String,
    pub currency_country: String,
    pub currency_units: i32,
    pub amount: i32,
    pub traded_at: NaiveDateTime,
    pub timezone: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = bookkeeper_statements)]
pub struct New<'a> {
    pub ledger_id: i32,
    pub transaction_id: i32,
    pub transaction_memo: &'a str,
    pub entry_id: i32,
    pub entry_memo: &'a str,
    pub entry_sn: &'a str,
    pub category_id: i32,
    pub category_label: &'a str,
    pub merchant_id: i32,
    pub merchant_label: &'a str,
    pub debtor_id: i32,
    pub debtor_label: &'a str,
    pub debtor_opening_balance: i32,
    pub debtor_closing_balance: i32,
    pub creditor_id: i32,
    pub creditor_label: &'a str,
    pub creditor_opening_balance: i32,
    pub creditor_closing_balance: i32,
    pub currency_id: i32,
    pub currency_code: &'a str,
    pub currency_name: &'a str,
    pub currency_country: &'a str,
    pub currency_units: i32,
    pub amount: i32,
    pub traded_at: NaiveDateTime,
    pub timezone: &'a str,
}
pub trait Dao {
    fn create(&mut self, form: &New) -> Result<()>;
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
    fn create(&mut self, form: &New) -> Result<()> {
        insert_into(bookkeeper_statements::dsl::bookkeeper_statements)
            .values(form)
            .execute(self)?;
        Ok(())
    }

    fn latest(&mut self, account: i32) -> Result<Option<Item>> {
        match bookkeeper_statements::dsl::bookkeeper_statements
            .filter(
                bookkeeper_statements::dsl::debtor_id
                    .eq(account)
                    .or(bookkeeper_statements::dsl::creditor_id.eq(account)),
            )
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
