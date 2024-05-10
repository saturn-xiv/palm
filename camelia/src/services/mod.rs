use std::any::type_name;
use std::fmt::Display;

use hibiscus::{cache::redis::ClusterConnection as Cache, session::Session};
use hyper::StatusCode;
use palm::{
    camelia::v1::{user_details::Type as UserProviderType, UserTokenAction},
    gourd::{
        protocols::{ROLE_ADMINISTRATOR, ROLE_ROOT},
        Policy,
    },
    HttpError, Jwt, Result,
};

use super::{
    models::user::{session::Dao as UserSessionDao, Dao as UserDao, Item as User},
    orm::postgresql::Connection as Db,
    NAME,
};

pub trait CurrentUserAdapter {
    fn current_user<P: Jwt>(
        &self,
        db: &mut Db,
        ch: &mut Cache,
        jwt: &P,
    ) -> Result<(User, String, (UserProviderType, i64))>;
}

impl CurrentUserAdapter for Session {
    fn current_user<P: Jwt>(
        &self,
        db: &mut Db,
        _ch: &mut Cache,
        jwt: &P,
    ) -> Result<(User, String, (UserProviderType, i64))> {
        if let Some(ref token) = self.token {
            let (_, uid) = jwt.verify(token, NAME, UserTokenAction::SignIn.as_str_name())?;
            let su = UserSessionDao::by_uid(db, &uid)?;
            let provider_type =
                UserProviderType::from_str_name(&su.provider_type).ok_or(Box::new(HttpError(
                    StatusCode::BAD_REQUEST,
                    Some(format!("bad provider type({})", su.provider_type)),
                )))?;
            let iu = UserDao::by_id(db, su.user_id)?;
            iu.available()?;

            return Ok((iu, su.uid, (provider_type, su.provider_id)));
        }

        Err(Box::new(HttpError(
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            None,
        )))
    }
}

impl User {
    pub fn has<P: Policy>(&self, policy: &P, role: &str) -> Result<()> {
        let ok = policy.has(self.id, role)?;
        if ok {
            return Ok(());
        }

        Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some(format!("user didn't has role {}", role)),
        )))
    }
    pub fn is_administrator<P: Policy>(&self, policy: &P) -> Result<()> {
        self.has(policy, ROLE_ADMINISTRATOR)
    }
    pub fn is_root<P: Policy>(&self, policy: &P) -> Result<()> {
        self.has(policy, ROLE_ROOT)
    }
    pub fn can_<P: Policy, O: Display>(
        &self,
        policy: &P,
        operation: O,
        resource_type: &str,
        resource_id: Option<i64>,
    ) -> Result<()> {
        let operation = operation.to_string();
        let ok = policy.can(self.id, &operation, resource_type, resource_id)?;
        if ok {
            return Ok(());
        }
        Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some(format!(
                "user didn't has permission({}, {}://{})",
                operation,
                resource_type,
                resource_id.map_or_else(|| "".to_string(), |x| x.to_string())
            )),
        )))
    }
    pub fn can<P: Policy, O: Display, R>(
        &self,
        policy: &P,
        operation: O,
        resource_id: Option<i64>,
    ) -> Result<()> {
        let resource_type = type_name::<R>();
        self.can_(policy, operation, resource_type, resource_id)
    }
}
