use chrono::NaiveDateTime;
use diesel::Connection as DieselConnection;
use juniper::GraphQLObject;
use validator::Validate;

use super::super::{
    Error, Jwt, Result,
    cache::redis::StandaloneConnection as Cache,
    models::locale::{Dao as LocaleDao, Item as LocaleItem},
    orm::postgresql::Connection as Db,
    rbac::Rbac,
};
use super::{Page, Pagination, Session};

pub async fn destroy<R: Rbac, J: Jwt>(
    ss: &Session,
    db: &mut Db,
    cache: &mut Cache,
    rbac: &R,
    jwt: &J,
    id: i32,
) -> Result<()> {
    let current_user = ss.current_user(db, cache, jwt).await?;
    rbac.is_administrator(current_user.id()).await?;

    db.transaction::<_, Error, _>(|tx| {
        LocaleDao::delete(tx, id as i64)?;
        Ok(())
    })?;

    Ok(())
}

#[derive(Clone, Debug, Validate)]
pub struct Set {
    #[validate(length(min = 2, max = 7), email)]
    pub lang: String,
    #[validate(length(min = 1, max = 255), email)]
    pub code: String,
    #[validate(length(min = 1))]
    pub message: String,
}
impl Set {
    pub async fn execute<R: Rbac, J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
    ) -> Result<()> {
        self.validate()?;
        let lang = self.lang.parse()?;

        let current_user = ss.current_user(db, cache, jwt).await?;
        rbac.is_administrator(current_user.id()).await?;

        db.transaction::<_, Error, _>(|tx| {
            match LocaleDao::by_lang_and_code(tx, &lang, &self.code) {
                Ok(it) => LocaleDao::update(tx, it.id, &self.message),
                Err(_) => LocaleDao::create(tx, &lang, &self.code, &self.message),
            }?;
            Ok(())
        })?;

        Ok(())
    }
}

pub fn by_lang(db: &mut Db, lang: &str) -> Result<Vec<Item>> {
    let lang = lang.parse()?;
    let items = LocaleDao::by_lang(db, &lang)?;
    Ok(items.into_iter().map(|x| x.into()).collect())
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "Locale")]
pub struct Item {
    pub id: i32,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub updated_at: NaiveDateTime,
}

impl From<LocaleItem> for Item {
    fn from(it: LocaleItem) -> Self {
        Self {
            id: it.id as i32,
            lang: it.lang.clone(),
            code: it.code.clone(),
            message: it.message.clone(),
            updated_at: it.updated_at,
        }
    }
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "IndexLocaleResponse")]
pub struct Index {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}

impl Index {
    pub async fn new<R: Rbac, J: Jwt>(
        ss: &Session,
        db: &mut Db,
        cache: &mut Cache,
        rbac: &R,
        jwt: &J,
        page: &Page,
    ) -> Result<Self> {
        {
            let current_user = ss.current_user(db, cache, jwt).await?;
            rbac.is_administrator(current_user.id()).await?;
        }

        let total = LocaleDao::count(db)?;
        let items = LocaleDao::index(db, page.offset(total), page.size())?;

        Ok(Self {
            items: items.into_iter().map(|x| x.into()).collect(),
            pagination: Pagination::new(page, total),
        })
    }
}
