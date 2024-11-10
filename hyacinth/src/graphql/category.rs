use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::session::current_user;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt, orm::postgresql::Pool as DbPool, session::Session, Error,
    HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::models::{
    category::{Dao as CategoryDao, Item as Category},
    ledger::Dao as LedgerDao,
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperCategory")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub parent_id: Option<i32>,
    pub label: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<Category> for Item {
    fn from(it: Category) -> Self {
        Self {
            id: it.id,
            ledger_id: it.ledger_id,
            parent_id: it.parent_id,
            label: it.label.clone(),
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        }
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

        for it in CategoryDao::by_ledger(db, id)? {
            items.push(it.into());
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
        let it = CategoryDao::by_id(db, id)?;
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        CategoryDao::disable(db, id)?;
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
        let it = CategoryDao::by_id(db, id)?;
        let ledger = LedgerDao::by_id(db, it.ledger_id)?;
        ledger.can_append(&user, enforcer).await?;
    }

    db.transaction::<_, Error, _>(|db| {
        CategoryDao::enable(db, id)?;
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
        {
            let (_, user) = current_user(ss, db, jwt)?;
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
            let it = CategoryDao::by_id(db, id)?;
            let ledger = LedgerDao::by_id(db, it.ledger_id)?;
            ledger.can_append(&user, enforcer).await?;
        }
        db.transaction::<_, Error, _>(|db| {
            CategoryDao::set_label(db, id, &self.label)?;
            Ok(())
        })?;
        Ok(())
    }
}
