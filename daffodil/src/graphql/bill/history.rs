use camelia::{
    models::user::Details as UserDetails, orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};
use casbin::Enforcer;
use chrono::NaiveDateTime;
use juniper::GraphQLObject;
use palm::{cache::redis::ClusterConnection as Cache, jwt::Jwt, session::Session, Result};
use tokio::sync::Mutex;

use super::super::super::models::{
    bill::history::{Dao as BillHistoryDao, Item as BillHistory},
    ledger::Dao as LedgerDao,
};

#[derive(GraphQLObject)]
#[graphql(name = "DaffodilIndexBillHistoryResponseItem")]
pub struct IndexResponseItem {
    pub id: i32,
    pub user: UserDetails,
    pub summary: String,
    pub price: i32,
    pub currency: String,
    pub merchant: String,
    pub category: String,
    pub paid_at: NaiveDateTime,
    pub paid_by: String,
    pub reason: String,
    pub created_at: NaiveDateTime,
}

impl IndexResponseItem {
    pub fn new(db: &mut Db, x: &BillHistory) -> Result<Self> {
        let it = Self {
            id: x.id,
            user: UserDetails::new(db, x.user_id)?,
            summary: x.summary.clone(),
            price: x.price.0 as i32,
            currency: x.currency.clone(),
            merchant: x.merchant.clone(),
            category: x.category.clone(),
            paid_at: x.paid_at,
            paid_by: x.paid_by.clone(),
            reason: x.reason.clone(),
            created_at: x.created_at,
        };
        Ok(it)
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "DaffodilIndexBillHistoryResponse")]
pub struct IndexResponse {
    pub items: Vec<IndexResponseItem>,
}

impl IndexResponse {
    pub async fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        ledger: i32,
    ) -> Result<Self> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        let ledger = LedgerDao::by_id(db, ledger)?;
        ledger.can_show(enf, &user).await?;

        let mut items = Vec::new();
        for it in BillHistoryDao::by_ledger(db, ledger.id)?.iter() {
            items.push(IndexResponseItem::new(db, it)?);
        }

        Ok(Self { items })
    }
}
