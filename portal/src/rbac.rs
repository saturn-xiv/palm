use hyacinth::rbac_v1::{
    PermissionView, Subject, UserRoleRequest,
    subject::{
        Role, User,
        role::{Administrator, Root},
    },
};
use hyper::StatusCode;
use juniper::{GraphQLEnum, GraphQLObject};

use super::{Dahlia, HttpError, Result};

pub trait Rbac {
    fn is_root(&self, user: i64) -> impl Future<Output = Result<()>>;
    fn is_administrator(&self, user: i64) -> impl Future<Output = Result<()>>;
    fn has_role(&self, user: i64, role: &str) -> impl Future<Output = Result<()>>;
    fn roles(&self, user: i64) -> impl Future<Output = Result<Vec<String>>>;
    fn permissions(&self, user: i64) -> impl Future<Output = Result<Vec<Permission>>>;
}

impl Rbac for Dahlia {
    async fn is_root(&self, user: i64) -> Result<()> {
        let mut it = Role::default();
        it.set_root(Root::default());
        self.has(user, it).await
    }
    async fn is_administrator(&self, user: i64) -> Result<()> {
        let mut it = Role::default();
        it.set_administrator(Administrator::default());
        self.has(user, it).await
    }
    async fn has_role(&self, user: i64, role: &str) -> Result<()> {
        let mut it = Role::default();
        it.set_code(role);
        self.has(user, it).await
    }

    async fn roles(&self, user: i64) -> Result<Vec<String>> {
        let mut req = User::default();
        req.set_id(user);
        let mut items = Vec::new();
        for it in self
            .enforcer
            .get_implicit_roles_for_user(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?
            .items()
            .iter()
        {
            if it.has_code() {
                let it = it.code().to_str()?;
                items.push(it.to_string());
            }
        }
        Ok(items)
    }
    async fn permissions(&self, user: i64) -> Result<Vec<Permission>> {
        let mut req = Subject::default();
        req.user_mut().set_id(user);
        let mut items = Vec::new();
        for it in self
            .enforcer
            .get_implicit_permissions(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?
            .items()
            .iter()
        {
            if let Ok(it) = Permission::new(it) {
                items.push(it);
            }
        }

        Ok(items)
    }
}

impl Dahlia {
    async fn has(&self, user: i64, role: Role) -> Result<()> {
        let mut req = UserRoleRequest::default();
        req.set_role(role);
        req.set_user({
            let mut it = User::default();
            it.set_id(user);
            it
        });

        self.enforcer
            .has_role_for_user(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        Ok(())
    }
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "Resource")]
pub struct Resource {
    pub r#type: String,
    pub id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, GraphQLEnum)]
#[graphql(name = "Action")]
pub enum Action {
    Read,
    Write,
    Append,
    Execute,
    Credit,
    Debit,
    Inquiry,
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "Permission")]
pub struct Permission {
    pub action: Action,
    pub resource: Resource,
}

impl Permission {
    pub fn new(it: PermissionView) -> Result<Self> {
        let it = Self {
            action: if it.action().has_read() {
                Ok(Action::Read)
            } else if it.action().has_write() {
                Ok(Action::Write)
            } else if it.action().has_append() {
                Ok(Action::Append)
            } else if it.action().has_execute() {
                Ok(Action::Execute)
            } else if it.action().has_credit() {
                Ok(Action::Credit)
            } else if it.action().has_debit() {
                Ok(Action::Debit)
            } else if it.action().has_inquiry() {
                Ok(Action::Inquiry)
            } else {
                Err(Box::new(HttpError(
                    StatusCode::LOCKED,
                    Some("Unsupported action".to_string()),
                )))
            }?,
            resource: Resource {
                r#type: it.object().r#type().to_string(),
                id: if it.object().has_all() {
                    Ok(None)
                } else if it.object().has_id() {
                    Ok(Some(it.object().id() as i32))
                } else {
                    Err(Box::new(HttpError(
                        StatusCode::NOT_IMPLEMENTED,
                        Some("Unsupported resource id".to_string()),
                    )))
                }?,
            },
        };
        Ok(it)
    }
}
