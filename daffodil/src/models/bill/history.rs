use camelia::orm::postgresql::Connection;
use chrono::NaiveDateTime;
use diesel::{data_types::Cents, insert_into, prelude::*};
use palm::Result;

use super::super::super::schema::daffodil_bills_history;
use super::Item as Bill;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub bill_id: i32,
    pub user_id: i32,
    pub summary: String,
    pub price: Cents,
    pub currency: String,
    pub merchant: String,
    pub category: String,
    pub paid_at: NaiveDateTime,
    pub paid_by: String,
    pub reason: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>>;
    fn by_bill(&mut self, bill: i32) -> Result<Vec<Item>>;
    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>>;
    fn create(&mut self, bill: &Bill, reason: &str) -> Result<()>;
}

impl Dao for Connection {
    fn by_user(&mut self, user: i32) -> Result<Vec<Item>> {
        Ok(daffodil_bills_history::dsl::daffodil_bills_history
            .filter(daffodil_bills_history::dsl::user_id.eq(user))
            .order(daffodil_bills_history::dsl::created_at.desc())
            .load::<Item>(self)?)
    }
    fn by_bill(&mut self, bill: i32) -> Result<Vec<Item>> {
        Ok(daffodil_bills_history::dsl::daffodil_bills_history
            .filter(daffodil_bills_history::dsl::bill_id.eq(bill))
            .order(daffodil_bills_history::dsl::created_at.desc())
            .load::<Item>(self)?)
    }
    fn by_ledger(&mut self, ledger: i32) -> Result<Vec<Item>> {
        Ok(daffodil_bills_history::dsl::daffodil_bills_history
            .filter(daffodil_bills_history::dsl::ledger_id.eq(ledger))
            .order(daffodil_bills_history::dsl::created_at.desc())
            .load::<Item>(self)?)
    }
    fn create(&mut self, bill: &Bill, reason: &str) -> Result<()> {
        insert_into(daffodil_bills_history::dsl::daffodil_bills_history)
            .values((
                daffodil_bills_history::dsl::ledger_id.eq(bill.ledger_id),
                daffodil_bills_history::dsl::user_id.eq(bill.user_id),
                daffodil_bills_history::dsl::bill_id.eq(bill.id),
                daffodil_bills_history::dsl::summary.eq(&bill.summary),
                daffodil_bills_history::dsl::price.eq(bill.price),
                daffodil_bills_history::dsl::currency.eq(&bill.currency),
                daffodil_bills_history::dsl::category.eq(&bill.category),
                daffodil_bills_history::dsl::merchant.eq(&bill.merchant),
                daffodil_bills_history::dsl::paid_at.eq(&bill.paid_at),
                daffodil_bills_history::dsl::paid_by.eq(&bill.paid_by),
                daffodil_bills_history::dsl::reason.eq(reason),
            ))
            .execute(self)?;
        Ok(())
    }
}
