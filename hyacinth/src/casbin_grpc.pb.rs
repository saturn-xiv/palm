/// Generated client implementations.
pub mod casbin_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// The Casbin service definition.
    #[derive(Debug, Clone)]
    pub struct CasbinClient<T> {
        channel: T,
    }

    impl<T> CasbinClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn new_enforcer<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::NewEnforcerReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::NewEnforcerRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/NewEnforcer", request)
        }

        pub fn new_adapter<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::NewAdapterReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::NewAdapterRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/NewAdapter", request)
        }

        pub fn enforce<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::EnforceRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/Enforce", request)
        }

        pub fn load_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::EmptyReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::EmptyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/LoadPolicy", request)
        }

        pub fn save_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::EmptyReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::EmptyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/SavePolicy", request)
        }

        pub fn add_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/AddPolicy", request)
        }

        pub fn add_named_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/AddNamedPolicy", request)
        }

        pub fn remove_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/RemovePolicy", request)
        }

        pub fn remove_named_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/RemoveNamedPolicy", request)
        }

        pub fn remove_filtered_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::FilteredPolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/RemoveFilteredPolicy", request)
        }

        pub fn remove_filtered_named_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::FilteredPolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/RemoveFilteredNamedPolicy", request)
        }

        pub fn get_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Array2DReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::EmptyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetPolicy", request)
        }

        pub fn get_named_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Array2DReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetNamedPolicy", request)
        }

        pub fn get_filtered_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Array2DReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::FilteredPolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetFilteredPolicy", request)
        }

        pub fn get_filtered_named_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Array2DReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::FilteredPolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetFilteredNamedPolicy", request)
        }

        pub fn add_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/AddGroupingPolicy", request)
        }

        pub fn add_named_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/AddNamedGroupingPolicy", request)
        }

        pub fn remove_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/RemoveGroupingPolicy", request)
        }

        pub fn remove_named_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/RemoveNamedGroupingPolicy", request)
        }

        pub fn remove_filtered_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::FilteredPolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/RemoveFilteredGroupingPolicy", request)
        }

        pub fn remove_filtered_named_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::FilteredPolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/RemoveFilteredNamedGroupingPolicy", request)
        }

        pub fn get_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Array2DReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::EmptyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetGroupingPolicy", request)
        }

        pub fn get_named_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Array2DReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetNamedGroupingPolicy", request)
        }

        pub fn get_filtered_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Array2DReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::FilteredPolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetFilteredGroupingPolicy", request)
        }

        pub fn get_filtered_named_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Array2DReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::FilteredPolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetFilteredNamedGroupingPolicy", request)
        }

        pub fn get_all_subjects<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::EmptyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetAllSubjects", request)
        }

        pub fn get_all_named_subjects<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::SimpleGetRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetAllNamedSubjects", request)
        }

        pub fn get_all_objects<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::EmptyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetAllObjects", request)
        }

        pub fn get_all_named_objects<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::SimpleGetRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetAllNamedObjects", request)
        }

        pub fn get_all_actions<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::EmptyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetAllActions", request)
        }

        pub fn get_all_named_actions<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::SimpleGetRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetAllNamedActions", request)
        }

        pub fn get_all_roles<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::EmptyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetAllRoles", request)
        }

        pub fn get_all_named_roles<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::SimpleGetRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetAllNamedRoles", request)
        }

        pub fn has_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/HasPolicy", request)
        }

        pub fn has_named_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/HasNamedPolicy", request)
        }

        pub fn has_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/HasGroupingPolicy", request)
        }

        pub fn has_named_grouping_policy<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PolicyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/HasNamedGroupingPolicy", request)
        }

        pub fn get_roles_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetRolesForUser", request)
        }

        pub fn get_implicit_roles_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetImplicitRolesForUser", request)
        }

        pub fn get_users_for_role<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ArrayReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetUsersForRole", request)
        }

        pub fn has_role_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/HasRoleForUser", request)
        }

        pub fn add_role_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/AddRoleForUser", request)
        }

        pub fn delete_role_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/DeleteRoleForUser", request)
        }

        pub fn delete_roles_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/DeleteRolesForUser", request)
        }

        pub fn delete_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/DeleteUser", request)
        }

        pub fn delete_role<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::EmptyReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/DeleteRole", request)
        }

        pub fn get_permissions_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Array2DReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PermissionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetPermissionsForUser", request)
        }

        pub fn get_implicit_permissions_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Array2DReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PermissionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/GetImplicitPermissionsForUser", request)
        }

        pub fn delete_permission<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PermissionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/DeletePermission", request)
        }

        pub fn add_permission_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PermissionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/AddPermissionForUser", request)
        }

        pub fn delete_permission_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PermissionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/DeletePermissionForUser", request)
        }

        pub fn delete_permissions_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PermissionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/DeletePermissionsForUser", request)
        }

        pub fn has_permission_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BoolReply>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PermissionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.casbin.v1.Casbin/HasPermissionForUser", request)
        }
    }
}
