use chrono::NaiveDateTime;
use juniper::GraphQLObject;

use super::super::{
    Jwt, Rbac, Result,
    cache::redis::StandaloneConnection as Cache,
    models::locale::{Dao as LocaleDao, Item as LocaleItem},
    orm::postgresql::Connection as Db,
};
use super::{Page, Pagination, Session};

pub fn by_lang(db: &mut Db, lang: &str) -> Result<Vec<Item>> {
    let items = LocaleDao::by_lang(db, lang)?;

    // let mut reply = Vec::new();
    // for it in items {
    //     reply.push(it.into());
    // }
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
            rbac.is_administrator(current_user.id).await?;
        }

        let total = LocaleDao::count(db)?;
        let items = LocaleDao::index(db, page.offset(total), page.size())?;

        Ok(Self {
            items: items.into_iter().map(|x| x.into()).collect(),
            pagination: Pagination::new(page, total),
        })
    }
}
