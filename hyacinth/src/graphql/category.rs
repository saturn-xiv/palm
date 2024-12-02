use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::session::current_user;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    session::Session,
    Error, HttpError, Result,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    category::{Dao as CategoryDao, Item as Category},
    ledger::Dao as LedgerDao,
    log::{Action, Dao as LogDao},
};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[graphql(name = "BookkeeperCategory")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub parent: Option<String>,
    pub label: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn new(db: &mut Db, it: &Category) -> Result<Self> {
        let it = Self {
            id: it.id,
            ledger_id: it.ledger_id,
            parent: match it.parent_id {
                Some(id) => {
                    let it = CategoryDao::by_id(db, id)?;
                    Some(it.label)
                }
                None => None,
            },
            label: it.label.clone(),
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

        for it in CategoryDao::by_ledger(db, id)?.iter() {
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
    let it = CategoryDao::by_id(db, id)?;
    {
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        CategoryDao::disable(db, id)?;
        LogDao::create(
            db,
            it.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::CreateCategory,
                &format!("disable category {}({})", it.label, it.id),
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
    let it = CategoryDao::by_id(db, id)?;
    {
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        CategoryDao::enable(db, id)?;
        LogDao::create(
            db,
            it.ledger_id,
            (user.id, &si.to_string()),
            (
                Action::EnableCategory,
                &format!("disable category {}({})", it.label, it.id),
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
}

impl Form {
    pub async fn create(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        ledger: i32,
        parent: Option<i32>,
    ) -> Result<()> {
        self.validate()?;
        let mut db = db.get()?;
        let db = db.deref_mut();

        let (si, user) = current_user(ss, db, jwt)?;
        {
            let ledger = LedgerDao::by_id(db, ledger)?;
            ledger.can_append(&user, enforcer).await?;
        }
        if let Some(id) = parent {
            let it = CategoryDao::by_id(db, id)?;
            if it.ledger_id != ledger {
                return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
            }
        }

        db.transaction::<_, Error, _>(|db| {
            CategoryDao::create(db, ledger, parent, &self.label)?;
            LogDao::create(
                db,
                ledger,
                (user.id, &si.to_string()),
                (
                    Action::CreateCategory,
                    &format!("create category {}", self.label),
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

        let (si, user) = current_user(ss, db, jwt)?;
        let it = CategoryDao::by_id(db, id)?;
        {
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
        db.transaction::<_, Error, _>(|db| {
            CategoryDao::set_label(db, id, &self.label)?;
            LogDao::create(
                db,
                it.ledger_id,
                (user.id, &si.to_string()),
                (
                    Action::UpdateCategory,
                    &format!("update category {}({})", self.label, id),
                    None,
                ),
                &ss.client_ip,
            )?;
            Ok(())
        })?;
        Ok(())
    }
}
