use std::any::type_name;
use std::fmt::{self, Display};
use std::str::FromStr;

use data_encoding::BASE64URL_NOPAD;
use hibiscus::{
    cache::redis::ClusterConnection as Cache,
    env::Thrift,
    gourd::{
        protocols::{ROLE_ADMINISTRATOR, ROLE_ROOT},
        Policy,
    },
    jwt::Jwt,
    session::Session,
    Error, HttpError, Result,
};
use hyper::StatusCode;
use log::error;
use serde::{Deserialize, Serialize};

use super::{
    models::user::{
        session::{Dao as UserSessionDao, ProviderType as UserProviderType},
        Action, Dao as UserDao, Item as User,
    },
    orm::postgresql::Connection as Db,
    NAME,
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
    pub fn has(&self, gourd: &Thrift, role: &str) -> Result<()> {
        let ok = Policy::has(gourd, self.id, role)?;
        if ok {
            return Ok(());
        }

        Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some(format!("{} didn't has role {}", self.nickname, role)),
        )))
    }
    pub fn is_administrator(&self, gourd: &Thrift) -> Result<()> {
        self.has(gourd, ROLE_ADMINISTRATOR)
    }
    pub fn is_root(&self, gourd: &Thrift) -> Result<()> {
        self.has(gourd, ROLE_ROOT)
    }
    pub fn can_<O: Display>(
        &self,
        gourd: &Thrift,
        operation: O,
        resource_type: &str,
        resource_id: Option<i64>,
    ) -> Result<()> {
        let operation = operation.to_string();
        let ok = Policy::can(gourd, self.id, &operation, resource_type, resource_id)?;
        if ok {
            return Ok(());
        }
        Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some(format!(
                "user didn't has permission({}, {}://{})",
                operation,
                resource_type,
                resource_id.map_or_else("".to_string(), |x| x.to_string())
            )),
        )))
    }
    pub fn can<R, O: Display>(
        &self,
        gourd: &Thrift,
        operation: O,
        resource_id: Option<i64>,
    ) -> Result<()> {
        let resource_type = type_name::<R>();
        self.can_(gourd, operation, resource_type, resource_id)
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
