use std::fmt::{self, Display};
use std::str::FromStr;

use casbin::{Enforcer, RbacApi};
use data_encoding::BASE64URL_NOPAD;
use hyper::StatusCode;
use log::error;
use palm::{
    cache::redis::ClusterConnection as Cache, has_permission, has_role, jwt::Jwt,
    rbac::v1 as rbac_v1, session::Session, to_code, Error, HttpError, Result, NAME,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use super::{
    models::user::{
        session::{Dao as UserSessionDao, ProviderType as UserProviderType},
        Action, Dao as UserDao, Item as User,
    },
    orm::postgresql::Connection as Db,
};

pub trait CurrentUserAdapter {
    fn current_user<P: Jwt>(
        &self,
        db: &mut Db,
        ch: &mut Cache,
        jwt: &P,
    ) -> Result<(User, String, (UserProviderType, i32))>;
}

impl CurrentUserAdapter for Session {
    fn current_user<P: Jwt>(
        &self,
        db: &mut Db,
        _ch: &mut Cache,
        jwt: &P,
    ) -> Result<(User, String, (UserProviderType, i32))> {
        if let Some(ref token) = self.token {
            let (_, uid) = jwt.verify(token, NAME, &Action::SignIn.to_string())?;
            let su = UserSessionDao::by_uid(db, &uid)?;
            let iu = UserDao::by_id(db, su.user_id)?;
            iu.available()?;
            return Ok((iu, su.uid, (su.provider_type.parse()?, su.provider_id)));
        }

        Err(Box::new(HttpError(
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            None,
        )))
    }
}

impl User {
    pub async fn has(&self, enforcer: &Mutex<Enforcer>, role: &rbac_v1::Role) -> Result<()> {
        {
            let role = role.to_string();
            let user = self.to_subject();
            if has_role!(enforcer, &user, &role) {
                return Ok(());
            }
        }
        Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some(format!("{} didn't has role {}", self.nickname, role)),
        )))
    }
    pub async fn is_member(&self, enforcer: &Mutex<Enforcer>, name: &str) -> Result<()> {
        self.has(
            enforcer,
            &rbac_v1::Role {
                by: Some(rbac_v1::role::By::Member(name.to_string())),
            },
        )
        .await
    }
    pub async fn is_administrator(&self, enforcer: &Mutex<Enforcer>) -> Result<()> {
        self.has(
            enforcer,
            &rbac_v1::Role {
                by: Some(rbac_v1::role::By::Administrator(())),
            },
        )
        .await
    }
    pub async fn is_root(&self, enforcer: &Mutex<Enforcer>) -> Result<()> {
        self.has(
            enforcer,
            &rbac_v1::Role {
                by: Some(rbac_v1::role::By::Root(())),
            },
        )
        .await
    }
    pub async fn can<R, O: Display>(
        &self,
        enforcer: &Mutex<Enforcer>,
        operation: O,
        resource_id: Option<i32>,
    ) -> Result<()> {
        if self.is_administrator(enforcer).await.is_ok() {
            return Ok(());
        }
        let user = self.to_subject();
        let action = {
            let it = operation.to_string();
            to_code!(it)
        };

        let object = rbac_v1::Resource::by_i::<R>(resource_id).to_string();
        if has_permission!(enforcer, &user, &object, &action) {
            return Ok(());
        }

        Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some(format!(
                "{} didn't has permission ({}, {})",
                self.nickname, operation, object
            )),
        )))
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug, Clone)]
pub struct Oauth2State {
    pub user: Option<String>,
    pub id: String,
}

impl fmt::Display for Oauth2State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let it = {
            let buf = flexbuffers::to_vec(self).map_err(|e| {
                error!("{:?}", e);
                fmt::Error
            })?;
            BASE64URL_NOPAD.encode(&buf)
        };
        write!(f, "{}", it)
    }
}

impl FromStr for Oauth2State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let buf = BASE64URL_NOPAD.decode(s.as_bytes())?;
        let it = flexbuffers::from_slice(&buf)?;
        Ok(it)
    }
}
