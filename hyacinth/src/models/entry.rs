use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use petunia::{orm::postgresql::Connection, Result};
use serde::{Deserialize, Serialize};

use super::super::schema::bookkeeper_entries;

#[derive(Hash, Eq, PartialEq, Queryable, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub transaction_id: i32,
    pub from_account_id: i32,
    pub to_account_id: i32,
    pub category_id: i32,
    pub merchant_id: i32,
    pub amount: i32,
    pub memo: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn create(
        &mut self,
        transaction: i32,
        category: i32,
        accounts: (i32, i32),
        merchant: (i32, i32, &str),
    ) -> Result<()>;
    fn by_id(&mut self, id: i32) -> Result<Item>;
    fn by_transaction(&mut self, transaction: i32) -> Result<Vec<Item>>;
    fn by_account(&mut self, account: i32) -> Result<Vec<Item>>;
    fn by_from_account(&mut self, account: i32) -> Result<Vec<Item>>;
    fn by_to_account(&mut self, account: i32) -> Result<Vec<Item>>;
    fn by_merchant(&mut self, merchant: i32) -> Result<Vec<Item>>;
    fn disable(&mut self, id: i32) -> Result<()>;
    fn enable(&mut self, id: i32) -> Result<()>;
}

impl Dao for Connection {
    fn create(
        &mut self,
        transaction: i32,
        category: i32,
        (from_account, to_account): (i32, i32),
        (merchant, amount, memo): (i32, i32, &str),
    ) -> Result<()> {
        insert_into(bookkeeper_entries::dsl::bookkeeper_entries)
            .values((
                bookkeeper_entries::dsl::transaction_id.eq(transaction),
                bookkeeper_entries::dsl::category_id.eq(category),
                bookkeeper_entries::dsl::from_account_id.eq(from_account),
                bookkeeper_entries::dsl::to_account_id.eq(to_account),
                bookkeeper_entries::dsl::merchant_id.eq(merchant),
                bookkeeper_entries::dsl::amount.eq(amount),
                bookkeeper_entries::dsl::memo.eq(memo),
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
            .order(bookkeeper_entries::dsl::created_at.desc())
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
            .order(bookkeeper_entries::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_from_account(&mut self, account: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_entries::dsl::bookkeeper_entries
            .filter(bookkeeper_entries::dsl::from_account_id.eq(account))
            .order(bookkeeper_entries::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_to_account(&mut self, account: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_entries::dsl::bookkeeper_entries
            .filter(bookkeeper_entries::dsl::to_account_id.eq(account))
            .order(bookkeeper_entries::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_merchant(&mut self, merchant: i32) -> Result<Vec<Item>> {
        let items = bookkeeper_entries::dsl::bookkeeper_entries
            .filter(bookkeeper_entries::dsl::merchant_id.eq(merchant))
            .order(bookkeeper_entries::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn disable(&mut self, id: i32) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it =
            bookkeeper_entries::dsl::bookkeeper_entries.filter(bookkeeper_entries::dsl::id.eq(id));
        update(it)
            .set((bookkeeper_entries::dsl::deleted_at.eq(&Some(now)),))
            .execute(self)?;
        Ok(())
    }
    fn enable(&mut self, id: i32) -> Result<()> {
        let it =
            bookkeeper_entries::dsl::bookkeeper_entries.filter(bookkeeper_entries::dsl::id.eq(id));
        update(it)
            .set((bookkeeper_entries::dsl::deleted_at.eq(&None::<NaiveDateTime>),))
            .execute(self)?;
        Ok(())
    }
}
