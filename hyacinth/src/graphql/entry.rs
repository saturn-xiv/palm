use std::ops::DerefMut;
use std::str::FromStr;

use casbin::Enforcer;
use chrono::{Duration, NaiveDateTime};
use chrono_tz::Tz;
use daffodil::{
    graphql::attachment::Item as Attachment,
    models::{
        attachment::Dao as AttachmentDao,
        currency::{Dao as CurrencyDao, Item as Currency},
    },
    session::current_user,
};
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::{GraphQLInputObject, GraphQLObject};
use petunia::{
    graphql::{DateTimePicker, Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    s3::Client as S3,
    session::Session,
    Error, HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use crate::models::{entry::Status, statement::Type};

use super::super::models::{
    account::Dao as AccountDao,
    category::Dao as CategoryDao,
    entry::{Dao as EntryDao, Item as Entry},
    ledger::Dao as LedgerDao,
    log::{Action, Dao as LogDao},
    merchant::Dao as MerchantDao,
    statement::Dao as StatementDao,
    transaction::{Dao as TransactionDao, Item as Transaction},
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperEntry")]
pub struct Item {
    pub id: i32,
    pub sn: String,
    pub transaction: super::transaction::Item,
    pub from_account: super::account::Item,
    pub to_account: super::account::Item,
    pub category: super::category::Item,
    pub merchant: super::merchant::Item,
    pub amount: i32,
    pub memo: String,
    pub bills: Vec<Attachment>,
    pub traded_at: DateTimePicker,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub async fn new(db: &mut Db, s3: &S3, it: &Entry, ttl: Option<Duration>) -> Result<Self> {
        let it = Self {
            id: it.id,
            sn: it.sn.clone(),
            transaction: {
                let it = TransactionDao::by_id(db, it.transaction_id)?;
                super::transaction::Item::new(&it)?
            },
            from_account: {
                let it = AccountDao::by_id(db, it.from_account_id)?;
                super::account::Item::new(db, &it)?
            },
            to_account: {
                let it = AccountDao::by_id(db, it.to_account_id)?;
                super::account::Item::new(db, &it)?
            },
            category: {
                let it = CategoryDao::by_id(db, it.category_id)?;
                super::category::Item::new(db, &it)?
            },
            merchant: {
                let it = MerchantDao::by_id(db, it.merchant_id)?;
                super::merchant::Item::new(db, &it)?
            },
            memo: it.memo.clone(),
            amount: it.amount,
            bills: {
                let mut items = Vec::new();
                for it in AttachmentDao::by_resource::<Entry>(db, Some(it.id))?.iter() {
                    let it = Attachment::new(s3, it, ttl).await?;
                    items.push(it);
                }
                items
            },
            traded_at: (it.traded_at, Tz::from_str(&it.timezone)?).try_into()?,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        };
        Ok(it)
    }
}

impl Item {
    pub async fn by_transaction(
        ss: &Session,
        db: &DbPool,
        s3: &S3,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        id: i32,
        ttl: Option<Duration>,
    ) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let it = LedgerDao::by_id(db, id)?;
            it.can_read(&user, enforcer).await?;
        }

        let mut items = Vec::new();

        for it in EntryDao::by_transaction(db, id)?.iter() {
            items.push(Item::new(db, s3, it, ttl).await?);
        }
        Ok(items)
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperList")]
pub struct List {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}
impl List {
    pub async fn by_ledger(
        ss: &Session,
        db: &DbPool,
        s3: &S3,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        (id, pager, ttl): (i32, &Pager, Option<Duration>),
    ) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let it = LedgerDao::by_id(db, id)?;
            it.can_read(&user, enforcer).await?;
        }

        let mut items = Vec::new();

        let total = EntryDao::count_by_ledger(db, id)?;
        for it in EntryDao::by_ledger(db, id, pager.offset(total), pager.size())?.iter() {
            items.push(Item::new(db, s3, it, ttl).await?);
        }
        Ok(Self {
            items,
            pagination: Pagination::new(pager, total),
        })
    }
}

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "NewBookkeeperEntryForm")]
pub struct New {
    pub from_account: i32,
    pub to_account: i32,
    pub category: i32,
    pub merchant: i32,
    pub amount: i32,
    #[validate(length(max = 1023))]
    pub memo: String,
    pub traded_at: String,
    pub timezone: String,
}

impl New {
    pub async fn create(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        transaction: i32,
    ) -> Result<()> {
        let picker = DateTimePicker {
            datetime: self.traded_at.clone(),
            timezone: self.timezone.clone(),
        };
        let (traded_at, timezone): (NaiveDateTime, Tz) = TryFrom::try_from(picker)?;
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();
        let (si, user) = current_user(ss, db, jwt)?;
        let it = {
            let it = TransactionDao::by_id(db, transaction)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("transaction is disabled".to_string()),
                )));
            }
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
            it
        };

        db.transaction::<_, Error, _>(|db| {
            self.save(
                db,
                &it,
                (traded_at, timezone),
                (user.id, &si.to_string()),
                &ss.client_ip,
            )?;

            Ok(())
        })?;
        Ok(())
    }

    fn check(&self, db: &mut Db, transaction: &Transaction) -> Result<Currency> {
        if self.from_account == self.to_account {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("can't trade by self".to_string()),
            )));
        }
        let from = {
            let it = AccountDao::by_id(db, self.from_account)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("from account is disabled".to_string()),
                )));
            }
            if it.ledger_id != transaction.ledger_id {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("from account's ledger not match".to_string()),
                )));
            }
            it
        };
        let to = {
            let it = AccountDao::by_id(db, self.to_account)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("to account is disabled".to_string()),
                )));
            }
            if it.ledger_id != transaction.ledger_id {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("to account's ledger not match".to_string()),
                )));
            }
            it
        };
        if from.currency_id != to.currency_id {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("accounts' currency not match".to_string()),
            )));
        }

        {
            let it = CategoryDao::by_id(db, self.category)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("category is disabled".to_string()),
                )));
            }
            if it.ledger_id != transaction.ledger_id {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("category's ledger not match".to_string()),
                )));
            }
        }
        {
            let it = MerchantDao::by_id(db, self.merchant)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("merchant is disabled".to_string()),
                )));
            }
            if it.ledger_id != transaction.ledger_id {
                return Err(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some("merchant's ledger not match".to_string()),
                )));
            }
        }

        let ic = CurrencyDao::by_id(db, from.currency_id)?;
        Ok(ic)
    }
    pub fn save(
        &self,
        db: &mut Db,
        transaction: &Transaction,
        traded_at: (NaiveDateTime, Tz),
        user: (i32, &str),
        client_ip: &str,
    ) -> Result<()> {
        let currency = self.check(db, transaction)?;
        EntryDao::create(
            db,
            (transaction.ledger_id, transaction.id, self.category),
            (self.from_account, self.to_account),
            (self.merchant, currency.id, self.amount, &self.memo),
            traded_at,
        )?;
        LogDao::create(
            db,
            transaction.ledger_id,
            user,
            (
                Action::CreateEntry,
                &format!(
                    "create entry {}({}, {}, {}) for transaction ({})",
                    self.memo, traded_at.0, currency.code, self.amount, transaction.id
                ),
                None,
            ),
            client_ip,
        )?;
        Ok(())
    }

    pub async fn update(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        id: i32,
    ) -> Result<()> {
        let picker = DateTimePicker {
            datetime: self.traded_at.clone(),
            timezone: self.timezone.clone(),
        };
        let (traded_at, timezone): (NaiveDateTime, Tz) = TryFrom::try_from(picker)?;
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();
        let (si, user) = current_user(ss, db, jwt)?;
        let (entry, transaction, currency) = {
            let ie = EntryDao::by_id(db, id)?;
            if Status::from_str(&ie.status)? == Status::Audited {
                return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
            }
            if ie.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("entry is disabled".to_string()),
                )));
            }

            let it = TransactionDao::by_id(db, ie.transaction_id)?;
            let ic = self.check(db, &it)?;
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;

            (ie, it, ic)
        };

        db.transaction::<_, Error, _>(|db| {
            EntryDao::update(
                db,
                id,
                self.category,
                (self.from_account, self.to_account),
                (self.merchant, currency.id, self.amount, &self.memo),
                (traded_at, timezone),
            )?;
            LogDao::create(
                db,
                transaction.ledger_id,
                (user.id, &si.to_string()),
                (
                    Action::CreateEntry,
                    &format!(
                        "update entry {:?} => {}({},{},{})",
                        entry, self.memo, traded_at, currency.code, self.amount,
                    ),
                    None,
                ),
                &ss.client_ip,
            )?;
            Ok(())
        })?;
        Ok(())
    }
}

pub async fn disable(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();

    let (si, user) = current_user(ss, db, jwt)?;
    let entry = EntryDao::by_id(db, id)?;
    if Status::from_str(&entry.status)? == Status::Audited {
        return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
    }
    let transaction = TransactionDao::by_id(db, entry.transaction_id)?;
    {
        let ledger = LedgerDao::by_id(db, transaction.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        EntryDao::disable(db, id)?;
        LogDao::create(
            db,
            transaction.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::DisableEntry,
                &format!("disable entry {}({})", entry.sn, entry.memo),
                None,
            ),
            &ss.client_ip,
        )?;
        Ok(())
    })?;

    Ok(())
}

pub async fn enable(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();

    let (si, user) = current_user(ss, db, jwt)?;
    let entry = EntryDao::by_id(db, id)?;
    if Status::from_str(&entry.status)? == Status::Audited {
        return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
    }
    let transaction = TransactionDao::by_id(db, entry.transaction_id)?;
    {
        let ledger = LedgerDao::by_id(db, transaction.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        EntryDao::enable(db, id)?;
        LogDao::create(
            db,
            transaction.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::EnableEntry,
                &format!("enable entry {}({})", entry.sn, entry.memo),
                None,
            ),
            &ss.client_ip,
        )?;
        Ok(())
    })?;

    Ok(())
}

pub async fn audit(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();

    let (si, user) = current_user(ss, db, jwt)?;
    let entry = EntryDao::by_id(db, id)?;

    if Status::from_str(&entry.status)? != Status::Pending {
        return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
    }
    if entry.amount < 0 {
        return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
    }

    let transaction = TransactionDao::by_id(db, entry.transaction_id)?;
    {
        let ledger = LedgerDao::by_id(db, transaction.ledger_id)?;
        ledger.can_credit(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        let timezone = Tz::from_str(&entry.timezone)?;
        {
            let closing = match StatementDao::latest(db, entry.from_account_id)? {
                Some(it) => it.closing_balance,
                None => 0,
            };
            StatementDao::create(
                db,
                (
                    entry.ledger_id,
                    entry.from_account_id,
                    entry.transaction_id,
                    entry.id,
                ),
                (entry.current_id, entry.amount, Type::Debit),
                (closing, closing - entry.amount),
                (entry.traded_at, timezone),
            )?;
        }
        {
            let closing = match StatementDao::latest(db, entry.to_account_id)? {
                Some(it) => it.closing_balance,
                None => 0,
            };
            StatementDao::create(
                db,
                (
                    entry.ledger_id,
                    entry.to_account_id,
                    entry.transaction_id,
                    entry.id,
                ),
                (entry.current_id, entry.amount, Type::Debit),
                (closing, closing + entry.amount),
                (entry.traded_at, timezone),
            )?;
        }
        EntryDao::set_status(db, entry.id, Status::Audited)?;
        LogDao::create(
            db,
            transaction.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::AuditEntry,
                &format!("audit entry {}({})", entry.sn, entry.memo),
                None,
            ),
            &ss.client_ip,
        )?;
        Ok(())
    })?;

    Ok(())
}
