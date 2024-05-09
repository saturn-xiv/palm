use casbin::Enforcer;
use chrono::NaiveDateTime;
use diesel::Connection as DieselConntection;
use hibiscus::{
    cache::redis::ClusterConnection as Cache,
    jwt::Jwt,
    pagination::{Pager, Pagination},
    session::Session,
    Error,
};
use juniper::GraphQLObject;
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::locale::{Dao as LocaleDao, Item as Locale},
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
};

#[derive(GraphQLObject)]
#[graphql(name = "LocaleIndexResponseItem")]
pub struct IndexResponseItem {
    pub id: i32,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub updated_at: NaiveDateTime,
}

impl From<Locale> for IndexResponseItem {
    fn from(x: Locale) -> Self {
        Self {
            id: x.id,
            lang: x.lang.clone(),
            code: x.code.clone(),
            message: x.message.clone(),
            updated_at: x.updated_at,
        }
    }
}

impl IndexResponseItem {
    pub fn by_lang(db: &mut Db, lang: &str) -> Result<Vec<Self>, Error> {
        let items = LocaleDao::by_lang(db, lang)?
            .into_iter()
            .map(|x| x.into())
            .collect();
        Ok(items)
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "LocaleIndexResponse")]
pub struct IndexResponse {
    pub items: Vec<IndexResponseItem>,
    pub pagination: Pagination,
}

impl IndexResponse {
    pub fn new(db: &mut Db, pager: &Pager) -> Result<Self, Error> {
        let total = LocaleDao::count(db)?;
        let items = LocaleDao::all(db, pager.offset(total), pager.size())?
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
    #[validate(length(min = 1, max = 7))]
    pub lang: String,
    #[validate(length(min = 1, max = 255))]
    pub code: String,
    #[validate(length(min = 1))]
    pub message: String,
}

impl Form {
    pub async fn handle<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
    ) -> Result<(), Error> {
        self.validate()?;
        let (user, _, _) = ss.current_user(db, ch, jwt)?;
        user.is_administrator(enf).await?;

        db.transaction::<_, Error, _>(move |db| {
            match LocaleDao::by_lang_and_code(db, &self.lang, &self.code) {
                Ok(ref it) => LocaleDao::update(db, it.id, &self.message)?,
                Err(_) => LocaleDao::create(db, &self.lang, &self.code, &self.message)?,
            };
            Ok(())
        })?;

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
    LocaleDao::delete(db, id)?;
    Ok(())
}
