pub mod info;

use std::ops::DerefMut;
use std::str::FromStr;

use casbin::{Enforcer, RbacApi};
use chrono_tz::Tz;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use language_tags::LanguageTag;
use petunia::{
    crypto::Key,
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    rbac::v1::{
        policy_roles_response::Item as PolicyRole, policy_users_response::Item as PolicyUser,
    },
    session::Session,
    themes::{Author, Layout},
    Error, HttpError, Result,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;
use validator::Validate;

use super::super::{
    models::{
        locale::{Dao as LocaleDao, I18n},
        log::{Dao as LogDao, Level as LogLevel},
        setting::Setting,
        user::{
            email::{Dao as EmailDao, Item as EmailUser},
            Dao as UserDao,
        },
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

#[derive(Validate)]
pub struct Install {
    pub user: super::user::email::SignUp,
    pub site: info::Base,
}

impl Install {
    pub async fn execute(
        &self,
        db: &DbPool,
        enforcer: &Mutex<Enforcer>,
        lang: &str,
        client_ip: &str,
    ) -> Result<()> {
        self.validate()?;
        let lang = {
            let it = LanguageTag::from_str(lang)?;
            it.to_string()
        };
        let timezone = {
            let it = Tz::from_str(&self.user.timezone)?;
            it.to_string()
        };
        let uid = Uuid::new_v4().to_string();

        let mut db = db.get()?;
        let db = db.deref_mut();

        let user = db.transaction::<_, Error, _>(|db| {
            if UserDao::total(db)? > 0 {
                return Err(Box::new(HttpError(StatusCode::BAD_REQUEST, None)));
            }
            I18n::set(db, &lang, Layout::TITLE, &self.site.title)?;
            I18n::set(db, &lang, Layout::SUBHEAD, &self.site.subhead)?;
            I18n::set(db, &lang, Layout::DESCRIPTION, &self.site.description)?;
            I18n::set(db, &lang, Layout::COPYRIGHT, &self.site.copyright)?;

            UserDao::create(db, &uid, &lang, &timezone)?;
            let user = UserDao::by_uid(db, &uid)?;
            EmailDao::create(
                db,
                user.id,
                &self.user.real_name,
                &self.user.nickname,
                &self.user.email,
                &self.user.password,
            )?;
            {
                let it = EmailDao::by_email(db, &self.user.email)?;
                EmailDao::confirm(db, it.id)?;
            }
            LogDao::create::<_, EmailUser>(
                db,
                user.id,
                super::NAME,
                LogLevel::Info,
                client_ip,
                None,
                "Init system administrator.",
            )?;
            Ok(user)
        })?;

        {
            let user = {
                let it = PolicyUser::by_id(user.id);
                it.to_string()
            };
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            enf.add_roles_for_user(
                &user,
                vec![
                    PolicyRole::administrator().to_string(),
                    PolicyRole::root().to_string(),
                ],
                None,
            )
            .await?;
        }

        Ok(())
    }
}
