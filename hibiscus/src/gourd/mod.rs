pub mod protocols;

use std::collections::{BTreeMap, BTreeSet};

use thrift::Result as ThriftResult;

use super::env::Thrift;

use self::protocols::{
    HealthSyncClient, Permission, PolicySyncClient, Resource, THealthSyncClient, TPolicySyncClient,
};

const POLICY_SERVICE_NAME: &str = "github.com/saturn-xiv/palm/gourd/services/v1/Policy";
const HEALTH_SERVICE_NAME: &str = "github.com/saturn-xiv/palm/gourd/services/v1/Health";

pub trait Policy {
    fn has(&self, user: i64, role: &str) -> ThriftResult<bool>;
    fn can(
        &self,
        user: i64,
        operation: &str,
        resource_type: &str,
        resource_id: Option<i64>,
    ) -> ThriftResult<bool>;
    fn delete_user(&self, user: i64) -> ThriftResult<()>;
    fn delete_role(&self, role: &str) -> ThriftResult<()>;
    fn get_roles_for_user(&self, user: i64) -> ThriftResult<BTreeSet<String>>;
    fn get_implicit_roles_for_user(&self, user: i64) -> ThriftResult<BTreeSet<String>>;
    fn get_users_for_role(&self, role: &str) -> ThriftResult<BTreeSet<i64>>;
    fn get_implicit_users_for_role(&self, role: &str) -> ThriftResult<BTreeSet<i64>>;
    fn add_roles_for_user(&self, user: i64, roles: &[&str]) -> ThriftResult<()>;
    fn delete_roles_for_user(&self, user: i64, roles: &[&str]) -> ThriftResult<()>;
    fn get_permissions_for_user(&self, user: i64) -> ThriftResult<BTreeSet<Permission>>;
    fn get_implicit_permissions_for_user(&self, user: i64) -> ThriftResult<BTreeSet<Permission>>;
    fn add_permissions_for_user(
        &self,
        user: i64,
        permissions: BTreeSet<Permission>,
    ) -> ThriftResult<()>;
    fn delete_permissions_for_user(
        &self,
        user: i64,
        permissions: BTreeSet<Permission>,
    ) -> ThriftResult<()>;
    fn get_permissions_for_role(&self, role: &str) -> ThriftResult<BTreeSet<Permission>>;
    fn get_implicit_permissions_for_role(&self, role: &str) -> ThriftResult<BTreeSet<Permission>>;
    fn add_permissions_for_role(
        &self,
        role: &str,
        permissions: BTreeSet<Permission>,
    ) -> ThriftResult<()>;
    fn delete_permissions_for_role(
        &self,
        role: &str,
        permissions: BTreeSet<Permission>,
    ) -> ThriftResult<()>;
}

impl Policy for Thrift {
    fn has(&self, user: i64, role: &str) -> ThriftResult<bool> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.has(user, role.to_string())
    }
    fn can(
        &self,
        user: i64,
        operation: &str,
        resource_type: &str,
        resource_id: Option<i64>,
    ) -> ThriftResult<bool> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.can(
            user,
            operation.to_string(),
            Resource {
                type_: resource_type.to_string(),
                id: resource_id,
            },
        )
    }
    fn delete_user(&self, user: i64) -> ThriftResult<()> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.delete_user(user)
    }
    fn delete_role(&self, role: &str) -> ThriftResult<()> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.delete_role(role.to_string())
    }
    fn get_roles_for_user(&self, user: i64) -> ThriftResult<BTreeSet<String>> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.get_roles_for_user(user)
    }
    fn get_implicit_roles_for_user(&self, user: i64) -> ThriftResult<BTreeSet<String>> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.get_implicit_roles_for_user(user)
    }
    fn get_users_for_role(&self, role: &str) -> ThriftResult<BTreeSet<i64>> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.get_users_for_role(role.to_string())
    }
    fn get_implicit_users_for_role(&self, role: &str) -> ThriftResult<BTreeSet<i64>> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.get_implicit_users_for_role(role.to_string())
    }
    fn add_roles_for_user(&self, user: i64, roles: &[&str]) -> ThriftResult<()> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        let roles: BTreeSet<String> = roles.iter().map(|x| x.to_string()).collect();
        client.add_roles_for_user(user, roles)
    }
    fn delete_roles_for_user(&self, user: i64, roles: &[&str]) -> ThriftResult<()> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        let roles: BTreeSet<String> = roles.iter().map(|x| x.to_string()).collect();
        client.delete_roles_for_user(user, roles)
    }
    fn get_permissions_for_user(&self, user: i64) -> ThriftResult<BTreeSet<Permission>> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.get_permissions_for_user(user)
    }
    fn get_implicit_permissions_for_user(&self, user: i64) -> ThriftResult<BTreeSet<Permission>> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.get_implicit_permissions_for_user(user)
    }
    fn add_permissions_for_user(
        &self,
        user: i64,
        permissions: BTreeSet<Permission>,
    ) -> ThriftResult<()> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.add_permissions_for_user(user, permissions)
    }
    fn delete_permissions_for_user(
        &self,
        user: i64,
        permissions: BTreeSet<Permission>,
    ) -> ThriftResult<()> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.delete_permissions_for_user(user, permissions)
    }
    fn get_permissions_for_role(&self, role: &str) -> ThriftResult<BTreeSet<Permission>> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.get_permissions_for_role(role.to_string())
    }
    fn get_implicit_permissions_for_role(&self, role: &str) -> ThriftResult<BTreeSet<Permission>> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.get_implicit_permissions_for_role(role.to_string())
    }
    fn add_permissions_for_role(
        &self,
        role: &str,
        permissions: BTreeSet<Permission>,
    ) -> ThriftResult<()> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.add_permissions_for_role(role.to_string(), permissions)
    }
    fn delete_permissions_for_role(
        &self,
        role: &str,
        permissions: BTreeSet<Permission>,
    ) -> ThriftResult<()> {
        let (i_prot, o_prot) = self.open(POLICY_SERVICE_NAME)?;
        let mut client: PolicySyncClient<_, _> = PolicySyncClient::new(i_prot, o_prot);
        client.delete_permissions_for_role(role.to_string(), permissions)
    }
}

pub trait Health {
    fn check(&self) -> ThriftResult<BTreeMap<String, String>>;
}

impl Health for Thrift {
    fn check(&self) -> ThriftResult<BTreeMap<String, String>> {
        let (i_prot, o_prot) = self.open(HEALTH_SERVICE_NAME)?;
        let mut client: HealthSyncClient<_, _> = HealthSyncClient::new(i_prot, o_prot);
        client.check()
    }
}
