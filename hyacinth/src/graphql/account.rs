use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::{
    graphql::currency::Item as Currency, models::currency::Dao as CurrencyDao,
    session::current_user,
};
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    session::Session,
    Error, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    account::{Dao as AccountDao, Item as Account, Type},
    ledger::Dao as LedgerDao,
    log::{Action, Dao as LogDao},
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperAccount")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub parent: Option<String>,
    pub label: String,
    pub memo: String,
    pub currency: Currency,
    pub r#type: Type,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn new(db: &mut Db, it: &Account) -> Result<Self> {
        let it = Self {
            id: it.id,
            ledger_id: it.ledger_id,
            parent: match it.parent_id {
                Some(id) => {
                    let it = AccountDao::by_id(db, id)?;
                    Some(it.label)
                }
                None => None,
            },
            label: it.label.clone(),
            memo: it.memo.clone(),
            currency: CurrencyDao::by_id(db, it.currency_id)?.into(),
            r#type: it.r#type.parse()?,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        };

        Ok(it)
    }
}

impl Item {
    pub async fn by_ledger(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        id: i32,
    ) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let ledger = LedgerDao::by_id(db, id)?;
            ledger.can_read(&user, enforcer).await?;
        }

        let mut items = Vec::new();
        for it in AccountDao::by_ledger(db, id)?.iter() {
            items.push(Self::new(db, it)?);
        }
        Ok(items)
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
    let it = AccountDao::by_id(db, id)?;
    {
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        AccountDao::disable(db, id)?;
        LogDao::create(
            db,
            it.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::DisableAccount,
                &format!("disable account {}({})", it.label, it.id),
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
    let it = AccountDao::by_id(db, id)?;
    {
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        AccountDao::enable(db, id)?;
        LogDao::create(
            db,
            it.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::EnableAccount,
                &format!("enable account {}({})", it.label, it.id),
                None,
            ),
            &ss.client_ip,
        )?;
        Ok(())
    })?;

    Ok(())
}

#[derive(Validate)]
pub struct Form {
    #[validate(length(min = 1, max = 63))]
    pub label: String,
    #[validate(length(max = 1023))]
    pub memo: String,
}

impl Form {
    pub async fn create_main(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        (ledger, type_, currency): (i32, Type, i32),
    ) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();

        let (si, user) = current_user(ss, db, jwt)?;
        {
            let ledger = LedgerDao::by_id(db, ledger)?;
            ledger.can_append(&user, enforcer).await?;
        }

        db.transaction::<_, Error, _>(|db| {
            AccountDao::create(db, ledger, None, &self.label, &self.memo, type_, currency)?;
            LogDao::create(
                db,
                ledger,
                (user.id, &si.to_string()),
                (
                    Action::CreateAccount,
                    &format!(
                        "create main-account {}({}, {})",
                        self.label, type_, self.memo
                    ),
                    None,
                ),
                &ss.client_ip,
            )?;
            Ok(())
        })?;
        Ok(())
    }
    pub async fn create_sub(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        (parent, type_, currency): (i32, Type, i32),
    ) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        let (si, user) = current_user(ss, db, jwt)?;
        let parent = AccountDao::by_id(db, parent)?;
        {
            let ledger = LedgerDao::by_id(db, parent.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
        db.transaction::<_, Error, _>(|db| {
            AccountDao::create(
                db,
                parent.ledger_id,
                Some(parent.id),
                &self.label,
                &self.memo,
                type_,
                currency,
            )?;
            LogDao::create(
                db,
                parent.ledger_id,
                (user.id, &si.to_string()),
                (
                    Action::CreateAccount,
                    &format!(
                        "create sub-account {}({},{}) for {}({})",
                        self.label, type_, self.memo, parent.label, parent.id
                    ),
                    None,
                ),
                &ss.client_ip,
            )?;
            Ok(())
        })?;
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
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let it = AccountDao::by_id(db, id)?;
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
        db.transaction::<_, Error, _>(|db| {
            AccountDao::set_details(db, id, &self.label, &self.memo)?;
            Ok(())
        })?;
        Ok(())
    }
}
