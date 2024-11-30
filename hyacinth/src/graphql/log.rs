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
    log::{Dao as LogDao, Item as Log},
};

#[derive(GraphQLObject)]
#[graphql(name = "BookkeeperLogItem")]
pub struct Item {
    pub id: i32,
    pub ledger_id: i32,
    pub user_id: i32,
    pub username: String,
    pub action: String,
    pub memo: String,
    pub reason: Option<String>,
    pub ip: String,
    pub created_at: NaiveDateTime,
}

impl From<Log> for Item {
    fn from(it: Log) -> Self {
        Self {
            id: it.id,
            ledger_id: it.ledger_id,
            user_id: it.user_id,
            username: it.username.clone(),
            action: it.action.clone(),
            memo: it.memo.clone(),
            reason: it.reason.clone(),
            ip: it.ip.clone(),
            created_at: it.created_at,
        }
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

        for it in LogDao::by_ledger(db, id, pager.offset(total), pager.size())? {
            items.push(it.into());
        }
        Ok(Self { items, pagination })
    }
}
