use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::{Duration, NaiveDateTime};
use chrono_tz::Tz;
use daffodil::{
    graphql::attachment::Item as Attachment,
    models::{
        attachment::Dao as AttachmentDao,
        currency::{Amount, Dao as CurrencyDao},
    },
    session::current_user,
};
use juniper::GraphQLObject;
use petunia::{
    graphql::{DateTimePicker, Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    s3::Client as S3,
    session::Session,
    Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    entry::Item as Entry,
    ledger::Dao as LedgerDao,
    statement::{Dao as StatementDao, Item as Statement},
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperStatement")]
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
    pub debtor_opening_balance: Amount,
    pub debtor_closing_balance: Amount,
    pub creditor_id: i32,
    pub creditor_label: String,
    pub creditor_opening_balance: Amount,
    pub creditor_closing_balance: Amount,
    pub amount: Amount,
    pub traded_at: NaiveDateTime,
    pub timezone: String,
    pub created_at: NaiveDateTime,
    pub bills: Vec<Attachment>,
}

impl Item {
    pub async fn new(db: &mut Db, s3: &S3, it: &Statement, ttl: Option<Duration>) -> Result<Self> {
        let currency = CurrencyDao::by_id(db, it.currency_id)?;
        let it = Self {
            id: it.id,
            ledger_id: it.ledger_id,
            transaction_id: it.transaction_id,
            transaction_memo: it.transaction_memo.clone(),
            entry_id: it.entry_id,
            entry_memo: it.entry_memo.clone(),
            entry_sn: it.entry_sn.clone(),
            category_id: it.category_id,
            category_label: it.category_label.clone(),
            merchant_id: it.merchant_id,
            merchant_label: it.merchant_label.clone(),
            debtor_id: it.debtor_id,
            debtor_label: it.debtor_label.clone(),
            debtor_opening_balance: Amount {
                value: it.debtor_opening_balance,
                currency: currency.clone(),
            },
            debtor_closing_balance: Amount {
                value: it.debtor_closing_balance,
                currency: currency.clone(),
            },
            creditor_id: it.creditor_id,
            creditor_label: it.creditor_label.clone(),
            creditor_opening_balance: Amount {
                value: it.creditor_opening_balance,
                currency: currency.clone(),
            },
            creditor_closing_balance: Amount {
                value: it.creditor_closing_balance,
                currency: currency.clone(),
            },
            amount: Amount {
                value: it.amount,
                currency,
            },
            traded_at: it.traded_at,
            timezone: it.timezone.clone(),
            created_at: it.created_at,
            bills: {
                let mut items = Vec::new();
                for it in AttachmentDao::by_resource::<Entry>(db, Some(it.id))?.iter() {
                    let it = Attachment::new(s3, it, ttl).await?;
                    items.push(it);
                }
                items
            },
        };

        Ok(it)
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperStatementList")]
pub struct List {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}

#[derive(Validate)]
pub struct ByLedger {
    #[validate(length(min = 1, max = 63))]
    pub begin: String,
    #[validate(length(min = 1, max = 63))]
    pub end: String,
    #[validate(length(min = 3, max = 31))]
    pub timezone: String,
    #[validate(range(min = 1, max = 48))]
    pub hours: i32,
}

impl ByLedger {
    pub async fn execute(
        &self,
        ss: &Session,
        (db, s3, jwt, enforcer): (&DbPool, &S3, &Jwt, &Mutex<Enforcer>),
        ledger: i32,
        pager: &Pager,
    ) -> Result<List> {
        self.validate()?;
        let (from, _): (NaiveDateTime, Tz) = {
            let it = DateTimePicker {
                datetime: self.begin.clone(),
                timezone: self.timezone.clone(),
            };
            it.try_into()?
        };
        let (to, _): (NaiveDateTime, Tz) = {
            let it = DateTimePicker {
                datetime: self.end.clone(),
                timezone: self.timezone.to_string(),
            };
            it.try_into()?
        };
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let it = LedgerDao::by_id(db, ledger)?;
            it.can_read(&user, enforcer).await?;
        }

        let total = StatementDao::count_by_ledger(db, ledger, from, to)?;
        let mut items = Vec::new();
        for it in
            StatementDao::by_ledger(db, ledger, from, to, pager.offset(total), pager.size())?.iter()
        {
            items.push(Item::new(db, s3, it, Some(Duration::hours(self.hours as i64))).await?);
        }
        Ok(List {
            items,
            pagination: Pagination::new(pager, total),
        })
    }
}
