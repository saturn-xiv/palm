use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use chrono_tz::Tz;
use daffodil::{
    models::currency::{Amount, Dao as CurrencyDao},
    session::current_user,
};
use juniper::GraphQLObject;
use petunia::{
    graphql::{DateTimePicker, Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    session::Session,
    Result,
};
use tokio::sync::Mutex;

use super::super::models::{
    account::Dao as AccountDao,
    entry::Dao as EntryDao,
    ledger::Dao as LedgerDao,
    statement::{Dao as StatementDao, Item as Statement},
    transaction::Dao as TransactionDao,
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperEntryDetailOfStatement")]
pub struct EntryDetail {
    pub id: i32,
    pub sn: String,
    pub memo: String,
    pub updated_at: NaiveDateTime,
}
#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperStatement")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub account: super::account::Item,
    pub transaction: super::transaction::Item,
    pub entry: EntryDetail,
    pub amount: Amount,
    pub opening_balance: Amount,
    pub closing_balance: Amount,
    pub traded_at: NaiveDateTime,
    pub timezone: String,
    pub create_at: NaiveDateTime,
}

impl Item {
    pub fn new(db: &mut Db, it: &Statement) -> Result<Self> {
        let currency = CurrencyDao::by_id(db, it.currency_id)?;
        let it = Self {
            id: it.id,
            ledger_id: it.ledger_id,
            account: {
                let it = AccountDao::by_id(db, it.account_id)?;
                super::account::Item::new(db, &it)?
            },
            transaction: {
                let it = TransactionDao::by_id(db, it.transaction_id)?;
                super::transaction::Item::new(&it)?
            },
            entry: {
                let it = EntryDao::by_id(db, it.entry_id)?;
                EntryDetail {
                    id: it.id,
                    sn: it.sn.clone(),
                    memo: it.memo.clone(),
                    updated_at: it.updated_at,
                }
            },
            amount: Amount {
                value: it.amount,
                currency: currency.clone(),
            },
            opening_balance: Amount {
                value: it.closing_balance,
                currency: currency.clone(),
            },
            closing_balance: Amount {
                value: it.closing_balance,
                currency: currency.clone(),
            },
            traded_at: it.traded_at,
            timezone: it.timezone.clone(),
            create_at: it.created_at,
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
impl List {
    pub async fn new(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        id: i32,
        (from, to, timezone): (String, String, String),
        pager: &Pager,
    ) -> Result<Self> {
        let (from, _): (NaiveDateTime, Tz) = {
            let it = DateTimePicker {
                datetime: from,
                timezone: timezone.clone(),
            };
            it.try_into()?
        };
        let (to, _): (NaiveDateTime, Tz) = {
            let it = DateTimePicker {
                datetime: to,
                timezone,
            };
            it.try_into()?
        };
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let it = LedgerDao::by_id(db, id)?;
            it.can_read(&user, enforcer).await?;
        }

        let total = StatementDao::count_by_ledger(db, id, from, to)?;
        let mut items = Vec::new();
        for it in
            StatementDao::by_ledger(db, id, from, to, pager.offset(total), pager.size())?.iter()
        {
            items.push(Item::new(db, it)?);
        }
        Ok(Self {
            items,
            pagination: Pagination::new(pager, total),
        })
    }
}
