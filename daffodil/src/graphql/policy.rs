use std::ops::DerefMut;

use casbin::{Enforcer, MgmtApi, RbacApi};
use hyper::StatusCode;
use juniper::GraphQLObject;
use petunia::{
    jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as DbPool,
    rbac::v1::{
        policy_roles_response::{item::By as RoleBy, Item as Role},
        policy_users_response::{item::Id as UserId, Item as User},
    },
    session::Session,
    HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    models::user::{Dao as UserDao, SelectOption as UserSelectOption},
    session::current_user,
};

#[derive(GraphQLObject)]
#[graphql(name = "PolicyUserRoleRelation")]
pub struct UserRoleRelation {
    pub users: Vec<UserSelectOption>,
    pub role: String,
}
impl UserRoleRelation {
    pub async fn all(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
    ) -> Result<Vec<Self>> {
        let mut db = db.get()?;
        let db = db.deref_mut();
        {
            let (_, user) = current_user(ss, db, jwt)?;
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            user.is_administrator(enf)?;
        }
        let mut items = Vec::new();
        {
            let mut enf = enforcer.lock().await;
            let enf = enf.deref_mut();
            for role in enf.get_all_roles().iter() {
                let it: Role = role.parse()?;
                if let Some(RoleBy::Code(role_code)) = it.by {
                    let mut users = Vec::new();
                    for user in enf.get_users_for_role(role, None).iter() {
                        let it: User = user.parse()?;
                        if let Some(UserId::I(id)) = it.id {
                            let it = UserSelectOption::new(db, id)?;
                            users.push(it);
                        }
                    }
                    items.push(Self {
                        role: role_code,
                        users,
                    })
                }
            }
        }
        Ok(items)
    }
}
pub async fn users(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
) -> Result<Vec<UserSelectOption>> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }
    let items = UserSelectOption::all(db)?;
    Ok(items)
}

pub async fn roles(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
) -> Result<Vec<String>> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }

    let mut items = Vec::new();
    {
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        for it in enf.get_all_roles().iter() {
            let it: Role = it.parse()?;
            if let Some(RoleBy::Code(code)) = it.by {
                items.push(code);
            }
        }
    }

    Ok(items)
}

#[derive(Validate)]
pub struct RoleForm {
    #[validate(length(min = 2, max = 63))]
    pub code: String,
}
impl RoleForm {
    pub async fn users(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
    ) -> Result<Vec<UserSelectOption>> {
        self.validate()?;
        _users_for_role(ss, db, jwt, enforcer, &Role::by_code(self.code.clone())).await
    }
    pub async fn apply_to_user(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        user: i32,
    ) -> Result<()> {
        self.validate()?;
        _add_role_for_user(
            ss,
            db,
            jwt,
            enforcer,
            user,
            &Role::by_code(self.code.clone()),
        )
        .await
    }
    pub async fn withdraw_from_user(
        &self,
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        user: i32,
    ) -> Result<()> {
        self.validate()?;
        _delete_role_for_user(
            ss,
            db,
            jwt,
            enforcer,
            user,
            &Role::by_code(self.code.clone()),
        )
        .await
    }
}

pub struct Administrator;

impl Administrator {
    pub async fn users(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
    ) -> Result<Vec<UserSelectOption>> {
        _users_for_role(ss, db, jwt, enforcer, &Role::administrator()).await
    }
    pub async fn apply_to_user(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        user: i32,
    ) -> Result<()> {
        _add_role_for_user(ss, db, jwt, enforcer, user, &Role::administrator()).await
    }
    pub async fn withdraw_from_user(
        ss: &Session,
        db: &DbPool,
        jwt: &Jwt,
        enforcer: &Mutex<Enforcer>,
        user: i32,
    ) -> Result<()> {
        _delete_role_for_user(ss, db, jwt, enforcer, user, &Role::administrator()).await
    }
}

async fn _add_role_for_user(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    user_id: i32,
    role: &Role,
) -> Result<()> {
    let role = role.to_string();
    let user = {
        let it = User::by_id(user_id);
        it.to_string()
    };
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
        {
            let it = UserDao::by_id(db, user_id)?;
            if it.is_root(enf).is_ok() {
                return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
            }
        }
    }
    {
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        enf.add_role_for_user(&user, &role, None).await?;
    }
    Ok(())
}

async fn _delete_role_for_user(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    user_id: i32,
    role: &Role,
) -> Result<()> {
    let role = role.to_string();
    let user = {
        let it = User::by_id(user_id);
        it.to_string()
    };
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;

        {
            let it = UserDao::by_id(db, user_id)?;
            if it.is_root(enf).is_ok() {
                return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
            }
        }
    }

    {
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        enf.delete_role_for_user(&user, &role, None).await?;
    }
    Ok(())
}
async fn _users_for_role(
    ss: &Session,
    db: &DbPool,
    jwt: &Jwt,
    enforcer: &Mutex<Enforcer>,
    role: &Role,
) -> Result<Vec<UserSelectOption>> {
    let role = role.to_string();
    let mut db = db.get()?;
    let db = db.deref_mut();
    {
        let (_, user) = current_user(ss, db, jwt)?;
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        user.is_administrator(enf)?;
    }

    let mut items = Vec::new();
    {
        let mut enf = enforcer.lock().await;
        let enf = enf.deref_mut();
        for it in enf.get_users_for_role(&role, None).iter() {
            let it: User = it.parse()?;
            if let Some(UserId::I(id)) = it.id {
                let it = UserSelectOption::new(db, id)?;
                items.push(it);
            }
        }
    }

    Ok(items)
}
