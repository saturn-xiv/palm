pub mod email;
pub mod google;
pub mod wechat;

use std::ops::DerefMut;
use std::str::FromStr;

use casbin::{Enforcer, RbacApi};
use chrono::{Duration, NaiveDateTime};
use diesel::Connection as DieselConnection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use petunia::{
    jwt::{openssl::OpenSsl as Jwt, Jwt as JwtProvider},
    orm::postgresql::{Connection as Db, Pool as DbPool},
    rbac::v1 as rbac_v1,
    session::Session,
    Error, HttpError, Result,
};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::super::{
    models::{
        log::{Dao as LogDao, Level as LogLevel},
        session::{Action as UserAction, Dao as SessionDao, ProviderType as UserProviderType},
        user::{Dao as UserDao, Item as User},
    },
    session::current_user,
};
use super::NAME;

#[derive(GraphQLObject)]
#[graphql(name = "UserSignInResponse")]
pub struct SignInResponse {
    pub real_name: String,
    pub provider_type: UserProviderType,
    pub token: String,
    pub lang: String,
    pub timezone: String,
    pub is_administrator: bool,
    pub is_root: bool,
    pub roles: Vec<String>,
}

impl SignInResponse {
    pub async fn new<T>(
        (user_id, real_name, provider_type, provider_id): (i32, String, UserProviderType, i32),
        db: &mut Db,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        client_ip: &str,
    ) -> Result<Self> {
        let uid = Uuid::new_v4().to_string();
        let ttl = Duration::days(1);

        let user = {
            let it = UserDao::by_id(db, user_id)?;
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::GONE,
                    Some("user is disabled".to_string()),
                )));
            }
            if it.locked_at.is_some() {
                return Err(Box::new(HttpError(
                    StatusCode::LOCKED,
                    Some("user is disabled".to_string()),
                )));
            }
            it
        };

        let (nbf, exp) = Jwt::timestamps(ttl);
        let token = jwt.sign(&uid, &UserAction::SignIn.to_string(), nbf, exp)?;
        let (is_administrator, is_root, roles) = {
            let mut enforcer = enforcer.lock().await;
            let enforcer = enforcer.deref_mut();

            let mut roles = Vec::new();
            {
                let user = {
                    let it = rbac_v1::policy_users_response::Item::by_id(user.id);
                    it.to_string()
                };
                for role in enforcer.get_implicit_roles_for_user(&user, None) {
                    let role = rbac_v1::policy_roles_response::Item::from_str(&role)?;
                    if let Some(rbac_v1::policy_roles_response::item::By::Code(code)) = role.by {
                        roles.push(code);
                    }
                }
            }
            (
                user.is_administrator(enforcer).is_ok(),
                user.is_root(enforcer).is_ok(),
                roles,
            )
        };

        db.transaction::<_, Error, _>(|db| {
            SessionDao::create(db, user_id, provider_type, provider_id, client_ip, ttl)?;
            LogDao::create::<_, T>(
                db,
                user_id,
                NAME,
                LogLevel::Info,
                client_ip,
                None,
                "Sign in.",
            )?;
            UserDao::sign_in(db, user_id, client_ip)?;
            Ok(())
        })?;
        Ok(Self {
            lang: user.lang.clone(),
            timezone: user.timezone,
            real_name,
            provider_type,
            token,
            is_administrator,
            is_root,
            roles,
        })
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "User")]
pub struct Item {
    pub id: i32,
    pub uid: String,
    pub lang: String,
    pub timezone: String,
    pub sign_in_count: i32,
    pub current_sign_in_at: Option<NaiveDateTime>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_at: Option<NaiveDateTime>,
    pub last_sign_in_ip: Option<String>,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<User> for Item {
    fn from(it: User) -> Self {
        Self {
            id: it.id,
            uid: it.uid.clone(),
            lang: it.lang.clone(),
            timezone: it.timezone.clone(),
            sign_in_count: it.sign_in_count,
            current_sign_in_at: it.current_sign_in_at,
            current_sign_in_ip: it.current_sign_in_ip.clone(),
            last_sign_in_at: it.last_sign_in_at,
            last_sign_in_ip: it.last_sign_in_ip.clone(),
            locked_at: it.locked_at,
            deleted_at: it.deleted_at,
            updated_at: it.updated_at,
        }
    }
}

pub async fn disable(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
    client_ip: &str,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }

    let it = UserDao::by_id(db, id)?;
    if it.deleted_at.is_some() {
        return Err(Box::new(HttpError(
            StatusCode::BAD_GATEWAY,
            Some(format!("User({}) is already disabled.", it.uid)),
        )));
    }
    {
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        if it.is_root(enf).is_ok() {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
    }
    db.transaction::<_, Error, _>(|db| {
        UserDao::disable(db, id)?;
        LogDao::create::<_, User>(
            db,
            id,
            NAME,
            LogLevel::Info,
            client_ip,
            None,
            "Disable by administrator.",
        )?;
        Ok(())
    })?;

    Ok(())
}
pub async fn enable(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
    client_ip: &str,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }

    let it = UserDao::by_id(db, id)?;
    if it.deleted_at.is_none() {
        return Err(Box::new(HttpError(
            StatusCode::BAD_GATEWAY,
            Some(format!("User({}) is already enabled.", it.uid)),
        )));
    }
    {
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        if it.is_root(enf).is_ok() {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
    }
    db.transaction::<_, Error, _>(|db| {
        UserDao::enable(db, id)?;
        LogDao::create::<_, User>(
            db,
            id,
            NAME,
            LogLevel::Info,
            client_ip,
            None,
            "Enable by administrator.",
        )?;
        Ok(())
    })?;

    Ok(())
}
pub async fn lock(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
    client_ip: &str,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }

    let it = UserDao::by_id(db, id)?;
    if it.locked_at.is_some() {
        return Err(Box::new(HttpError(
            StatusCode::BAD_GATEWAY,
            Some(format!("User({}) is already locked.", it.uid)),
        )));
    }
    {
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        if it.is_root(enf).is_ok() {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
    }
    db.transaction::<_, Error, _>(|db| {
        UserDao::lock(db, id)?;
        LogDao::create::<_, User>(
            db,
            id,
            NAME,
            LogLevel::Info,
            client_ip,
            None,
            "Lock by administrator.",
        )?;
        Ok(())
    })?;

    Ok(())
}
pub async fn unlock(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    id: i32,
    client_ip: &str,
) -> Result<()> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }

    let it = UserDao::by_id(db, id)?;
    if it.locked_at.is_none() {
        return Err(Box::new(HttpError(
            StatusCode::BAD_GATEWAY,
            Some(format!("User({}) isn't locked.", it.uid)),
        )));
    }
    {
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        if it.is_root(enf).is_ok() {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
    }
    db.transaction::<_, Error, _>(|db| {
        UserDao::unlock(db, id)?;
        LogDao::create::<_, User>(
            db,
            id,
            NAME,
            LogLevel::Info,
            client_ip,
            None,
            "Unlock by administrator.",
        )?;
        Ok(())
    })?;

    Ok(())
}
