use std::ops::DerefMut;
use std::str::FromStr;

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

use super::super::models::{
    account::Dao as AccountDao,
    category::Dao as CategoryDao,
    entry::{
        Dao as EntryDao, Item as Entry, New as NewEntry, Status as EntryStatus,
        Update as UpdateEntry,
    },
    ledger::Dao as LedgerDao,
    log::{Action, Dao as LogDao},
    merchant::Dao as MerchantDao,
    statement::{Dao as StatementDao, New as NewStatement},
    transaction::{Dao as TransactionDao, Item as Transaction},
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperEntry")]
pub struct Item {
    pub id: i32,
    pub sn: String,
    pub transaction: super::transaction::Item,
    pub debtor: super::account::Item,
    pub creditor: super::account::Item,
    pub category: super::category::Item,
    pub merchant: super::merchant::Item,
    pub amount: Amount,
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
            debtor: {
                let it = AccountDao::by_id(db, it.debtor_id)?;
                super::account::Item::new(db, &it)?
            },
            creditor: {
                let it = AccountDao::by_id(db, it.creditor_id)?;
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
            amount: Amount {
                currency: CurrencyDao::by_id(db, it.current_id)?,
                value: it.amount,
            },
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
    pub debtor: i32,
    pub creditor: i32,
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

    fn check(&self, db: &mut Db, transaction: &Transaction) -> Result<i32> {
        if self.amount <= 0 {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("amount couldn't be less than 0".to_string()),
            )));
        }
        if self.debtor == self.creditor {
            return Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("can't trade by self".to_string()),
            )));
        }
        let debtor = {
            let it = AccountDao::by_id(db, self.debtor)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("debtor account is disabled".to_string()),
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
        let creditor = {
            let it = AccountDao::by_id(db, self.creditor)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("creditor is disabled".to_string()),
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
        if debtor.currency_id != creditor.currency_id {
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

        Ok(debtor.currency_id)
    }
    fn save(
        &self,
        db: &mut Db,
        transaction: &Transaction,
        traded_at: (NaiveDateTime, Tz),
        user: (i32, &str),
        client_ip: &str,
    ) -> Result<()> {
        let currency = {
            let ic = self.check(db, transaction)?;
            CurrencyDao::by_id(db, ic)?
        };
        let sn = NewEntry::next_sn(db, transaction.ledger_id)?;
        EntryDao::create(
            db,
            &NewEntry::generate(
                transaction,
                (self.debtor, self.creditor),
                (self.category, self.merchant),
                (&sn, self.amount, &self.memo, currency.id),
                traded_at,
            ),
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
        let traded_at = TryFrom::try_from(picker)?;
        self.validate()?;

        let mut db = db.get()?;
        let db = db.deref_mut();
        let (si, user) = current_user(ss, db, jwt)?;
        let (entry, transaction, currency) = {
            let ie = EntryDao::by_id(db, id)?;
            if EntryStatus::from_str(&ie.status)? == EntryStatus::Audited {
                return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
            }
            if ie.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("entry is disabled".to_string()),
                )));
            }

            let it = TransactionDao::by_id(db, ie.transaction_id)?;
            let ic = {
                let ic = self.check(db, &it)?;
                CurrencyDao::by_id(db, ic)?
            };
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;

            (ie, it, ic)
        };

        db.transaction::<_, Error, _>(|db| {
            EntryDao::update(
                db,
                id,
                &UpdateEntry::new(
                    (self.debtor, self.creditor),
                    (self.category, self.merchant),
                    (self.amount, &self.memo, currency.id),
                    traded_at,
                ),
            )?;
            LogDao::create(
                db,
                transaction.ledger_id,
                (user.id, &si.to_string()),
                (
                    Action::CreateEntry,
                    &format!(
                        "update entry {:?} => {}({:?},{},{})",
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
    if EntryStatus::from_str(&entry.status)? == EntryStatus::Audited {
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
    if EntryStatus::from_str(&entry.status)? == EntryStatus::Audited {
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

    if EntryStatus::from_str(&entry.status)? != EntryStatus::Pending {
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
    let category = CategoryDao::by_id(db, entry.category_id)?;
    let merchant = MerchantDao::by_id(db, entry.merchant_id)?;
    let debtor = AccountDao::by_id(db, entry.debtor_id)?;
    let creditor = AccountDao::by_id(db, entry.creditor_id)?;
    let currency = CurrencyDao::by_id(db, entry.current_id)?;

    db.transaction::<_, Error, _>(|db| {
        {
            let debtor_closing = match StatementDao::latest(db, entry.debtor_id)? {
                Some(it) => it.debtor_closing_balance,
                None => 0,
            };
            let creditor_closing = match StatementDao::latest(db, entry.creditor_id)? {
                Some(it) => it.creditor_closing_balance,
                None => 0,
            };
            StatementDao::create(
                db,
                &NewStatement {
                    ledger_id: entry.ledger_id,
                    transaction_id: transaction.id,
                    transaction_memo: &transaction.memo,
                    category_id: category.id,
                    category_label: &category.label,
                    merchant_id: merchant.id,
                    merchant_label: &merchant.label,
                    entry_id: entry.id,
                    entry_memo: &entry.memo,
                    entry_sn: &entry.sn,
                    debtor_id: debtor.id,
                    debtor_label: &debtor.label,
                    debtor_opening_balance: debtor_closing,
                    debtor_closing_balance: debtor_closing - entry.amount,
                    creditor_id: creditor.id,
                    creditor_label: &creditor.label,
                    creditor_opening_balance: creditor_closing,
                    creditor_closing_balance: creditor_closing + entry.amount,
                    currency_id: currency.id,
                    currency_code: &currency.code,
                    currency_name: &currency.name,
                    currency_country: &currency.country,
                    currency_units: currency.units,
                    traded_at: entry.traded_at,
                    timezone: &entry.timezone,
                    amount: entry.amount,
                },
            )?;
        }

        EntryDao::set_status(db, entry.id, EntryStatus::Audited)?;
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
