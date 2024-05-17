pub mod trash;

use camelia::orm::postgresql::Connection;
use chrono::NaiveDateTime;
use diesel::{delete, insert_into, prelude::*};
use palm::{daffodil::v1, Result};
use serde::{Deserialize, Serialize};

use super::super::schema::daffodil_transactions;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub book_id: i64,
    pub source_account_id: i64,
    pub destination_account_id: i64,
    pub merchant_id: i64,
    pub type_: i32,
    pub amount: i64,
    pub currency: String,
    pub summary: String,
    pub paid_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_book(&mut self, book: i64) -> Result<Vec<Item>>;
    fn by_merchant(&mut self, merchant: i64) -> Result<Vec<Item>>;
    fn by_account(&mut self, account: i64) -> Result<Vec<Item>>;
    fn by_id(&mut self, id: i64) -> Result<Item>;
    fn first_by_book(&mut self, book: i64) -> Result<Item>;
    fn create(
        &mut self,
        user: i64,
        book: i64,
        accounts: (i64, i64),
        merchant: (
            i64,
            v1::transaction_index_response::item::Type,
            NaiveDateTime,
        ),
        amount: (i64, &str),
        summary: &str,
    ) -> Result<()>;
    fn delete(&mut self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn by_book(&mut self, book: i64) -> Result<Vec<Item>> {
        let items = daffodil_transactions::dsl::daffodil_transactions
            .filter(daffodil_transactions::dsl::book_id.eq(book))
            .order(daffodil_transactions::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_merchant(&mut self, merchant: i64) -> Result<Vec<Item>> {
        let items = daffodil_transactions::dsl::daffodil_transactions
            .filter(daffodil_transactions::dsl::merchant_id.eq(merchant))
            .order(daffodil_transactions::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_account(&mut self, account: i64) -> Result<Vec<Item>> {
        let items = daffodil_transactions::dsl::daffodil_transactions
            .filter(
                daffodil_transactions::dsl::source_account_id
                    .eq(account)
                    .or(daffodil_transactions::dsl::destination_account_id.eq(account)),
            )
            .order(daffodil_transactions::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn first_by_book(&mut self, book: i64) -> Result<Item> {
        let it = daffodil_transactions::dsl::daffodil_transactions
            .filter(daffodil_transactions::dsl::book_id.eq(book))
            .order(daffodil_transactions::dsl::created_at.asc())
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_id(&mut self, id: i64) -> Result<Item> {
        let it = daffodil_transactions::dsl::daffodil_transactions
            .filter(daffodil_transactions::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &mut self,
        user: i64,
        book: i64,
        (source_account, destination_account): (i64, i64),
        (merchant, type_, paid_at): (
            i64,
            v1::transaction_index_response::item::Type,
            NaiveDateTime,
        ),
        (amount, currency): (i64, &str),
        summary: &str,
    ) -> Result<()> {
        insert_into(daffodil_transactions::dsl::daffodil_transactions)
            .values((
                daffodil_transactions::dsl::user_id.eq(user),
                daffodil_transactions::dsl::book_id.eq(book),
                daffodil_transactions::dsl::source_account_id.eq(source_account),
                daffodil_transactions::dsl::destination_account_id.eq(destination_account),
                daffodil_transactions::dsl::merchant_id.eq(merchant),
                daffodil_transactions::dsl::type_.eq(type_ as i32),
                daffodil_transactions::dsl::amount.eq(amount),
                daffodil_transactions::dsl::currency.eq(currency),
                daffodil_transactions::dsl::summary.eq(summary),
                daffodil_transactions::dsl::paid_at.eq(paid_at),
            ))
            .execute(self)?;
        Ok(())
    }

    fn delete(&mut self, id: i64) -> Result<()> {
        delete(
            daffodil_transactions::dsl::daffodil_transactions
                .filter(daffodil_transactions::dsl::id.eq(id)),
        )
        .execute(self)?;
        Ok(())
    }
}
