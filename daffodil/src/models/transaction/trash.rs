use camelia::orm::postgresql::Connection;
use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use palm::Result;
use serde::{Deserialize, Serialize};

use super::super::super::schema::daffodil_transaction_trash;
use super::Item as Transaction;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub original_id: i64,
    pub user_id: i64,
    pub book_id: i64,
    pub source_account_id: i64,
    pub destination_account_id: i64,
    pub merchant_id: i64,
    pub type_: i32,
    pub amount: i64,
    pub currency: String,
    pub summary: String,
    pub original_created_at: NaiveDateTime,
    pub reason: String,
    pub operator_id: i64,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_book(&mut self, book: i64) -> Result<Vec<Item>>;
    fn create(&mut self, user: i64, item: &Transaction, reason: &str) -> Result<()>;
}

impl Dao for Connection {
    fn by_book(&mut self, book: i64) -> Result<Vec<Item>> {
        let items = daffodil_transaction_trash::dsl::daffodil_transaction_trash
            .filter(daffodil_transaction_trash::dsl::book_id.eq(book))
            .order(daffodil_transaction_trash::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn create(&mut self, user: i64, item: &Transaction, reason: &str) -> Result<()> {
        insert_into(daffodil_transaction_trash::dsl::daffodil_transaction_trash)
            .values((
                daffodil_transaction_trash::dsl::original_id.eq(item.id),
                daffodil_transaction_trash::dsl::user_id.eq(item.user_id),
                daffodil_transaction_trash::dsl::book_id.eq(item.book_id),
                daffodil_transaction_trash::dsl::source_account_id.eq(item.source_account_id),
                daffodil_transaction_trash::dsl::destination_account_id
                    .eq(item.destination_account_id),
                daffodil_transaction_trash::dsl::merchant_id.eq(item.merchant_id),
                daffodil_transaction_trash::dsl::type_.eq(item.type_),
                daffodil_transaction_trash::dsl::amount.eq(item.amount),
                daffodil_transaction_trash::dsl::currency.eq(&item.currency),
                daffodil_transaction_trash::dsl::summary.eq(&item.summary),
                daffodil_transaction_trash::dsl::original_created_at.eq(item.created_at),
                daffodil_transaction_trash::dsl::reason.eq(reason),
                daffodil_transaction_trash::dsl::operator_id.eq(user),
            ))
            .execute(self)?;
        Ok(())
    }
}
