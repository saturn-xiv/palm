pub mod info;

use std::ops::DerefMut;

use casbin::Enforcer;
use diesel::Connection as DieselConnection;
use petunia::{
    crypto::Key,
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    session::Session,
    themes::{Author, Layout},
    Error, Result,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;

use super::super::{
    models::{
        locale::{Dao as LocaleDao, I18n},
        setting::Setting,
    },
    session::current_user,
};

pub fn layout(ss: &Session, db: &DbPool, secrets: Key) -> Result<Layout> {
    let mut db_s = db.get()?;
    let db_s = db_s.deref_mut();
    let cipher = Result::<Vec<u8>>::from(secrets)?;
    let mut st = Setting::new(&cipher, db_s);

    let mut db = db.get()?;
    let db = db.deref_mut();

    let it = Layout {
        title: I18n::t(db, &ss.lang, Layout::TITLE, None::<String>),
        subhead: I18n::t(db, &ss.lang, Layout::SUBHEAD, None::<String>),
        description: I18n::t(db, &ss.lang, Layout::DESCRIPTION, None::<String>),
        copyright: I18n::t(db, &ss.lang, Layout::DESCRIPTION, None::<String>),
        author: {
            if let Ok(ref buf) = st.get(&info::Author::key(&ss.lang), None) {
                flexbuffers::from_slice(buf)?
            } else {
                Author::default()
            }
        },
        keywords: {
            if let Ok(ref buf) = st.get(&Layout::KEYWORDS.to_string(), None) {
                flexbuffers::from_slice(buf)?
            } else {
                Vec::new()
            }
        },
        locale: ss.lang.clone(),
        languages: LocaleDao::languages(db)?,
        cn_bi: None,
        cn_gab: None,
        cn_icp: None,
    };
    Ok(it)
}

pub fn get<T: DeserializeOwned>(
    db: &DbPool,
    secrets: Key,
    key: String,
    user: Option<i32>,
) -> Result<T> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let cipher = Result::<Vec<u8>>::from(secrets)?;
    let mut st = Setting::new(&cipher, db);
    let it = {
        let buf = st.get(&key, user)?;
        flexbuffers::from_slice(&buf)?
    };
    Ok(it)
}
pub async fn set<T: Serialize>(
    ss: &Session,
    db: &DbPool,
    secrets: Key,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    (key, user, value, encrypt): (String, Option<i32>, &T, bool),
) -> Result<()> {
    let value = flexbuffers::to_vec(value)?;
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }
    let cipher = Result::<Vec<u8>>::from(secrets)?;

    db.transaction::<_, Error, _>(|db| {
        let mut st = Setting::new(&cipher, db);
        st.set(&key, user, &value, encrypt)?;
        Ok(())
    })?;

    Ok(())
}
