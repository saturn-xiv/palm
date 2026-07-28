use hyacinth::rbac_v1::{
    UserRoleRequest,
    subject::{
        Role, User,
        role::{Administrator, Root},
    },
};

use super::{Dahlia, HttpError, Rbac, Result};

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
