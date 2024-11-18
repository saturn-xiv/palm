pub mod info;
pub mod seo;
pub mod smtp;

use std::any::type_name;
use std::ops::DerefMut;
use std::str::FromStr;

use casbin::{Enforcer, RbacApi};
use chrono_tz::Tz;
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use language_tags::LanguageTag;
use petunia::{
    crypto::Key,
    iso4217,
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::{Connection as Db, Pool as DbPool},
    rbac::v1::{
        policy_roles_response::Item as PolicyRole, policy_users_response::Item as PolicyUser,
    },
    session::Session,
    themes::{Author, Layout},
    Error, HttpError, Result,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;
use validator::Validate;

use super::super::{
    models::{
        currency::Dao as CurrencyDao,
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
use super::user::CurrentUser;

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[graphql(name = "RefreshResponse")]
pub struct Refresh {
    pub site_info: Layout,
    pub current_user: Option<CurrentUser>,
}

impl Refresh {
    pub async fn new(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        secrets: Key,
        enforcer: &Mutex<Enforcer>,
    ) -> Result<Self> {
        let mut db = db.get()?;
        let db = db.deref_mut();

        Ok(Self {
            site_info: Self::_site_info(ss, db, secrets).unwrap_or_default(),
            current_user: Self::_current_user(ss, db, jwt, enforcer).await.ok(),
            // current_user: Some(Self::_current_user(ss, db, jwt, enforcer).await?),
        })
    }
    async fn _current_user(
        ss: &Session,
        db: &mut Db,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
    ) -> Result<CurrentUser> {
        let (si, _) = current_user(ss, db, jwt)?;
        CurrentUser::new(
            (si.user_id, &si.real_name, si.provider_type.parse()?),
            db,
            enforcer,
        )
        .await
    }
    fn _site_info(ss: &Session, db: &mut Db, secrets: Key) -> Result<Layout> {
        let cipher = Result::<Vec<u8>>::from(secrets)?;
        let (favicon, author, keywords) = {
            let mut st = Setting::new(&cipher, db);
            let lg = if let Ok(ref buf) = st.get(&Layout::FAVICON.to_string(), None) {
                flexbuffers::from_slice(buf)?
            } else {
                "/my/logo.svg".to_string()
            };
            let ah = if let Ok(ref buf) = st.get(&type_name::<Author>().to_string(), None) {
                flexbuffers::from_slice(buf)?
            } else {
                Author::default()
            };
            let ks = if let Ok(ref buf) = st.get(&Layout::KEYWORDS.to_string(), None) {
                let it: info::Keywords = flexbuffers::from_slice(buf)?;
                it.items
            } else {
                Vec::new()
            };
            (lg, ah, ks)
        };

        let it = Layout {
            title: I18n::t(db, &ss.lang, Layout::TITLE, None::<String>),
            subhead: I18n::t(db, &ss.lang, Layout::SUBHEAD, None::<String>),
            description: I18n::t(db, &ss.lang, Layout::DESCRIPTION, None::<String>),
            copyright: I18n::t(db, &ss.lang, Layout::COPYRIGHT, None::<String>),
            favicon,
            author,
            keywords,
            locale: ss.lang.clone(),
            languages: LocaleDao::languages(db)?,

            // TODO
            cn_mps: None,
            cn_icp: None,
        };
        Ok(it)
    }
}

pub async fn get_<T: DeserializeOwned>(
    ss: &Session,
    db: &DbPool,
    secrets: Key,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    key: &str,
    user: Option<i32>,
) -> Result<T> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }
    let cipher = Result::<Vec<u8>>::from(secrets)?;
    let mut st = Setting::new(&cipher, db);
    let it = {
        let buf = st.get(&key, user)?;
        flexbuffers::from_slice(&buf)?
    };
    Ok(it)
}
pub async fn get<T: DeserializeOwned>(
    ss: &Session,
    db: &DbPool,
    secrets: Key,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    user: Option<i32>,
) -> Result<T> {
    get_(ss, db, secrets, jwt, enforcer, type_name::<T>(), user).await
}
pub async fn set_<T: Serialize + Validate>(
    ss: &Session,
    db: &DbPool,
    secrets: Key,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    (key, user, value, encrypt): (&str, Option<i32>, &T, bool),
) -> Result<()> {
    value.validate()?;
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

pub async fn set<T: Serialize + Validate>(
    ss: &Session,
    db: &DbPool,
    secrets: Key,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    (user, value, encrypt): (Option<i32>, &T, bool),
) -> Result<()> {
    set_(
        ss,
        db,
        secrets,
        jwt,
        enforcer,
        (type_name::<T>(), user, value, encrypt),
    )
    .await
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

        for it in iso4217::Currency::list_one()?.iter() {
            CurrencyDao::create(
                db,
                &it.code,
                &it.number,
                &it.name,
                &it.country,
                it.units as i32,
            )?;
        }
        Ok(())
    }
}
