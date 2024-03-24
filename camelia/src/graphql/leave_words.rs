use casbin::Enforcer;
use chrono::NaiveDateTime;
use juniper::GraphQLObject;
use palm::{
    cache::redis::ClusterConnection as Cache,
    jwt::Jwt,
    pagination::{Pager, Pagination},
    session::Session,
    Error, TextEditor,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::leave_word::{Dao as LeaveWordDao, Item as LeaveWord},
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};

#[derive(GraphQLObject)]
#[graphql(name = "LeaveWordIndexResponseItem")]
pub struct IndexResponseItem {
    pub id: i32,
    pub lang: String,
    pub ip: String,
    pub body: String,
    pub body_editor: String,
    pub status: String,
    pub published_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<LeaveWord> for IndexResponseItem {
    fn from(x: LeaveWord) -> Self {
        Self {
            id: x.id,
            lang: x.lang.clone(),
            ip: x.ip.clone(),
            body: x.body.clone(),
            body_editor: x.body_editor.clone(),
            status: x.status.clone(),
            published_at: x.published_at,
            deleted_at: x.deleted_at,
            updated_at: x.updated_at,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "LeaveWordIndexResponse")]
pub struct IndexResponse {
    pub items: Vec<IndexResponseItem>,
    pub pagination: Pagination,
}

impl IndexResponse {
    pub async fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        pager: &Pager,
    ) -> Result<Self, Error> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        user.is_administrator(enf).await?;

        let total = LeaveWordDao::count(db)?;
        let items = LeaveWordDao::all(db, pager.offset(total), pager.size())?
            .into_iter()
            .map(|x| x.into())
            .collect();

        Ok(Self {
            items,
            pagination: Pagination::new(pager, total),
        })
    }
}

#[derive(Validate)]
pub struct Form {
    #[validate(length(min = 15, max = 1023))]
    pub content: String,
    pub editor: TextEditor,
}

impl Form {
    pub async fn handle(&self, ss: &Session, db: &mut Db) -> Result<(), Error> {
        self.validate()?;
        LeaveWordDao::create(db, &ss.lang, &ss.client_ip, &self.content, &self.editor)?;
        Ok(())
    }
}

pub async fn destroy<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    id: i32,
) -> Result<(), Error> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;

    let it = LeaveWordDao::by_id(db, id)?;
    LeaveWordDao::destroy(db, it.id)?;

    Ok(())
}
