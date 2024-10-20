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
    session::{Dao as SessionDao, Item as SessionItem},
    user::{Action as UserAction, Dao as UserDao, Item as UserItem},
};

impl UserItem {
    pub fn is_root(&self, enforcer: &mut Enforcer) -> Result<()> {
        self.has_(enforcer, &Role::root())
    }
    pub fn is_administrator(&self, enforcer: &mut Enforcer) -> Result<()> {
        self.has_(enforcer, &Role::administrator())
    }
    pub fn has(&self, enforcer: &mut Enforcer, role: &str) -> Result<()> {
        self.has_(enforcer, &Role::by_code(role.to_string()))
    }
    pub fn has_(&self, enforcer: &mut Enforcer, role: &Role) -> Result<()> {
        let user = {
            let it = User::by_id(self.id);
            it.to_string()
        };
        let role = role.to_string();

        for it in enforcer.get_implicit_roles_for_user(&user, None) {
            if it == role {
                return Ok(());
            }
        }

        Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)))
    }
    pub fn can(
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

pub fn current_user(ss: &Session, db: &mut Db, jwt: &Jwt) -> Result<(SessionItem, UserItem)> {
    if let Some(ref token) = ss.token {
        let uid = jwt.verify(token, &UserAction::SignIn.to_string())?;
        let sit = SessionDao::by_uid(db, &uid)?;
        let uit = UserDao::by_id(db, sit.user_id)?;
        if uit.locked_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::LOCKED, None)));
        }
        if uit.deleted_at.is_some() {
            return Err(Box::new(HttpError(StatusCode::GONE, None)));
        }
        return Ok((sit, uit));
    }
    Err(Box::new(HttpError(StatusCode::NOT_FOUND, None)))
}
