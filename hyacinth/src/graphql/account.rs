use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::{
    graphql::currency::Item as Currency, models::currency::Dao as CurrencyDao,
    session::current_user,
};
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    session::Session,
    Error, HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    account::{Dao as AccountDao, Item as Account, Type},
    ledger::Dao as LedgerDao,
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperAccount")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub parent_id: Option<i32>,
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
            parent_id: it.parent_id,
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
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let it = AccountDao::by_id(db, id)?;
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        AccountDao::disable(db, id)?;
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
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let it = AccountDao::by_id(db, id)?;
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        AccountDao::enable(db, id)?;
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
    pub async fn create(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        (ledger, parent, currency, type_): (i32, Option<i32>, i32, Type),
    ) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let ledger = LedgerDao::by_id(db, ledger)?;
            ledger.can_append(&user, enforcer).await?;
        }
        if let Some(id) = parent {
            let it = AccountDao::by_id(db, id)?;
            if it.ledger_id != ledger {
                return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
            }
        }

        db.transaction::<_, Error, _>(|db| {
            AccountDao::create(db, ledger, parent, &self.label, &self.memo, type_, currency)?;
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
