use casbin::{Enforcer, RbacApi};
use hyper::StatusCode;
use petunia::{
    jwt::{openssl::OpenSsl as Jwt, Jwt as JwtProvider},
    orm::postgresql::Connection as Db,
    rbac::v1::{
        policy_permissions_response::item::{Operation, Resource},
        policy_roles_response::Item as Role,
        policy_users_response::Item as User,
    },
    session::Session,
    HttpError, Result,
};

use super::models::{
    session::Dao as SessionDao,
    user::{Dao as UserDao, Item as UserItem},
};

pub trait CurrentUser {
    fn is_root(&self, enforcer: &mut Enforcer) -> Result<()>;
    fn is_administrator(&self, enforcer: &mut Enforcer) -> Result<()>;
    fn has(&self, enforcer: &mut Enforcer, role: &str) -> Result<()>;
    fn has_(&self, enforcer: &mut Enforcer, role: &Role) -> Result<()>;
    fn can(
        &self,
        enforcer: &mut Enforcer,
        operation: &Operation,
        resource: &Resource,
    ) -> Result<()>;
}

impl CurrentUser for UserItem {
    fn is_root(&self, enforcer: &mut Enforcer) -> Result<()> {
        self.has_(enforcer, &Role::root())
    }
    fn is_administrator(&self, enforcer: &mut Enforcer) -> Result<()> {
        self.has_(enforcer, &Role::administrator())
    }
    fn has(&self, enforcer: &mut Enforcer, role: &str) -> Result<()> {
        self.has_(enforcer, &Role::by_code(role.to_string()))
    }
    fn has_(&self, enforcer: &mut Enforcer, role: &Role) -> Result<()> {
        let user = {
            let it = User::by_id(self.id);
            it.to_string()
        };
        let role = role.to_string();
        {
            for it in enforcer.get_implicit_roles_for_user(&user, None) {
                if it == role {
                    return Ok(());
                }
            }
        }
        Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
    }
    fn can(
        &self,
        enforcer: &mut Enforcer,
        operation: &Operation,
        resource: &Resource,
    ) -> Result<()> {
        if self._can(enforcer, operation, resource).is_ok() {
            return Ok(());
        }
        if resource.id.is_none() {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }
        let parent = Resource {
            r#type: resource.r#type.clone(),
            id: None,
        };
        self._can(enforcer, operation, &parent)
    }
}

impl UserItem {
    const SIGN_IN: &str = "users.sign-in";
    pub async fn new(ss: &Session, db: &mut Db, jwt: &Jwt) -> Result<Self> {
        if let Some(ref token) = ss.token {
            let uid = jwt.verify(token, Self::SIGN_IN)?;
            let sit = SessionDao::by_uid(db, &uid)?;
            let it = UserDao::by_id(db, sit.user_id)?;
            if it.locked_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
            }
            if it.deleted_at.is_some() {
                return Err(Box::new(HttpError(StatusCode::GONE, None)));
            }
        }
        Err(Box::new(HttpError(StatusCode::NOT_FOUND, None)))
    }
    fn _can(
        &self,
        enforcer: &mut Enforcer,
        operation: &Operation,
        resource: &Resource,
    ) -> Result<()> {
        let user = {
            let it = User::by_id(self.id);
            it.to_string()
        };
        let operation = operation.to_string();
        let resource = resource.to_string();
        {
            for permission in enforcer
                .get_implicit_permissions_for_user(&user, None)
                .iter()
            {
                if permission.len() == 3 && resource == permission[1] && operation == permission[2]
                {
                    return Ok(());
                }
            }
        }
        Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
    }
}
