use std::ops::DerefMut;

use casbin::Enforcer;
use chrono::NaiveDateTime;
use daffodil::session::current_user;
use juniper::GraphQLObject;
use petunia::{
    graphql::{Pager, Pagination},
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    Result,
};
use tokio::sync::Mutex;

use super::super::models::{
    ledger::Dao as LedgerDao,
    log::{Dao as LogDao, Detail, Item as Log},
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperLogItem")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub user_id: i32,
    pub detail: Detail,
    pub action: String,
    pub created_at: NaiveDateTime,
}

impl Item {
    pub fn new(it: &Log) -> Result<Self> {
        let detail = flexbuffers::from_slice(&it.detail)?;
        Ok(Self {
            detail,
            id: it.id,
            ledger_id: it.ledger_id,
            user_id: it.user_id,
            action: it.action.clone(),
            created_at: it.created_at,
        })
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperLogList")]
pub struct List {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}
impl List {
    pub async fn by_ledger(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        id: i32,
        pager: &Pager,
    ) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let it = LedgerDao::by_id(db, id)?;
            it.can_read(&user, enforcer).await?;
        }

        let mut items = Vec::new();
        let total = LogDao::count_by_ledger(db, id)?;
        let pagination = Pagination::new(pager, total);

        for it in LogDao::by_ledger(db, id, pager.page(total), pager.size())?.iter() {
            items.push(Item::new(it)?);
        }
        Ok(Self { items, pagination })
    }
}
