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

/// Generated server implementations.
pub mod casbin_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        // will trigger if compression is disabled
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;

    /// Generated trait containing gRPC methods that should be implemented for use with CasbinServer.

    #[async_trait]
    pub trait Casbin : std::marker::Send + std::marker::Sync + 'static {
        async fn new_enforcer(&self, request: tonic::Request<super::NewEnforcerRequest>)
            -> std::result::Result<tonic::Response<super::NewEnforcerReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn new_adapter(&self, request: tonic::Request<super::NewAdapterRequest>)
            -> std::result::Result<tonic::Response<super::NewAdapterReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn enforce(&self, request: tonic::Request<super::EnforceRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn load_policy(&self, request: tonic::Request<super::EmptyRequest>)
            -> std::result::Result<tonic::Response<super::EmptyReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn save_policy(&self, request: tonic::Request<super::EmptyRequest>)
            -> std::result::Result<tonic::Response<super::EmptyReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn add_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn add_named_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn remove_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn remove_named_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn remove_filtered_policy(&self, request: tonic::Request<super::FilteredPolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn remove_filtered_named_policy(&self, request: tonic::Request<super::FilteredPolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_policy(&self, request: tonic::Request<super::EmptyRequest>)
            -> std::result::Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_named_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_filtered_policy(&self, request: tonic::Request<super::FilteredPolicyRequest>)
            -> std::result::Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_filtered_named_policy(&self, request: tonic::Request<super::FilteredPolicyRequest>)
            -> std::result::Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn add_grouping_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn add_named_grouping_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn remove_grouping_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn remove_named_grouping_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn remove_filtered_grouping_policy(&self, request: tonic::Request<super::FilteredPolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn remove_filtered_named_grouping_policy(&self, request: tonic::Request<super::FilteredPolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_grouping_policy(&self, request: tonic::Request<super::EmptyRequest>)
            -> std::result::Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_named_grouping_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_filtered_grouping_policy(&self, request: tonic::Request<super::FilteredPolicyRequest>)
            -> std::result::Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_filtered_named_grouping_policy(&self, request: tonic::Request<super::FilteredPolicyRequest>)
            -> std::result::Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_all_subjects(&self, request: tonic::Request<super::EmptyRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_all_named_subjects(&self, request: tonic::Request<super::SimpleGetRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_all_objects(&self, request: tonic::Request<super::EmptyRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_all_named_objects(&self, request: tonic::Request<super::SimpleGetRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_all_actions(&self, request: tonic::Request<super::EmptyRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_all_named_actions(&self, request: tonic::Request<super::SimpleGetRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_all_roles(&self, request: tonic::Request<super::EmptyRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_all_named_roles(&self, request: tonic::Request<super::SimpleGetRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn has_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn has_named_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn has_grouping_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn has_named_grouping_policy(&self, request: tonic::Request<super::PolicyRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_roles_for_user(&self, request: tonic::Request<super::UserRoleRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_implicit_roles_for_user(&self, request: tonic::Request<super::UserRoleRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_users_for_role(&self, request: tonic::Request<super::UserRoleRequest>)
            -> std::result::Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn has_role_for_user(&self, request: tonic::Request<super::UserRoleRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn add_role_for_user(&self, request: tonic::Request<super::UserRoleRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn delete_role_for_user(&self, request: tonic::Request<super::UserRoleRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn delete_roles_for_user(&self, request: tonic::Request<super::UserRoleRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn delete_user(&self, request: tonic::Request<super::UserRoleRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn delete_role(&self, request: tonic::Request<super::UserRoleRequest>)
            -> std::result::Result<tonic::Response<super::EmptyReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_permissions_for_user(&self, request: tonic::Request<super::PermissionRequest>)
            -> std::result::Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn get_implicit_permissions_for_user(&self, request: tonic::Request<super::PermissionRequest>)
            -> std::result::Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn delete_permission(&self, request: tonic::Request<super::PermissionRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn add_permission_for_user(&self, request: tonic::Request<super::PermissionRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn delete_permission_for_user(&self, request: tonic::Request<super::PermissionRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn delete_permissions_for_user(&self, request: tonic::Request<super::PermissionRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        async fn has_permission_for_user(&self, request: tonic::Request<super::PermissionRequest>)
            -> std::result::Result<tonic::Response<super::BoolReply>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }

    /// The Casbin service definition.

    #[derive(Debug)]
    pub struct CasbinServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }

    impl<T> CasbinServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }

        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }

        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }

        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }

        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }

        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }

        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }

    impl<T, B> tonic::codegen::Service<http::Request<B>> for CasbinServer<T>
        where
            T: Casbin,
            B: Body + std::marker::Send + 'static,
            B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/palm.casbin.v1.Casbin/NewEnforcer" => {
                    #[allow(non_camel_case_types)]
                    struct new_enforcerSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::NewEnforcerRequest> for new_enforcerSvc<T> {
                        type Response = super::NewEnforcerReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::NewEnforcerRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::new_enforcer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = new_enforcerSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/NewAdapter" => {
                    #[allow(non_camel_case_types)]
                    struct new_adapterSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::NewAdapterRequest> for new_adapterSvc<T> {
                        type Response = super::NewAdapterReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::NewAdapterRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::new_adapter(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = new_adapterSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/Enforce" => {
                    #[allow(non_camel_case_types)]
                    struct enforceSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::EnforceRequest> for enforceSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::EnforceRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::enforce(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = enforceSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/LoadPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct load_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for load_policySvc<T> {
                        type Response = super::EmptyReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::EmptyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::load_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = load_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/SavePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct save_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for save_policySvc<T> {
                        type Response = super::EmptyReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::EmptyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::save_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = save_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/AddPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct add_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for add_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::add_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = add_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/AddNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct add_named_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for add_named_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::add_named_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = add_named_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/RemovePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct remove_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for remove_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::remove_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = remove_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/RemoveNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct remove_named_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for remove_named_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::remove_named_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = remove_named_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/RemoveFilteredPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct remove_filtered_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest> for remove_filtered_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::FilteredPolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::remove_filtered_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = remove_filtered_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/RemoveFilteredNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct remove_filtered_named_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest> for remove_filtered_named_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::FilteredPolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::remove_filtered_named_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = remove_filtered_named_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct get_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for get_policySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::EmptyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct get_named_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for get_named_policySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_named_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_named_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetFilteredPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct get_filtered_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest> for get_filtered_policySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::FilteredPolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_filtered_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_filtered_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetFilteredNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct get_filtered_named_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest> for get_filtered_named_policySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::FilteredPolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_filtered_named_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_filtered_named_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/AddGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct add_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for add_grouping_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::add_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = add_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/AddNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct add_named_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for add_named_grouping_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::add_named_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = add_named_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/RemoveGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct remove_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for remove_grouping_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::remove_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = remove_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/RemoveNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct remove_named_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for remove_named_grouping_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::remove_named_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = remove_named_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/RemoveFilteredGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct remove_filtered_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest> for remove_filtered_grouping_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::FilteredPolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::remove_filtered_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = remove_filtered_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/RemoveFilteredNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct remove_filtered_named_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest> for remove_filtered_named_grouping_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::FilteredPolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::remove_filtered_named_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = remove_filtered_named_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct get_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for get_grouping_policySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::EmptyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct get_named_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for get_named_grouping_policySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_named_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_named_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetFilteredGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct get_filtered_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest> for get_filtered_grouping_policySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::FilteredPolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_filtered_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_filtered_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetFilteredNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct get_filtered_named_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest> for get_filtered_named_grouping_policySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::FilteredPolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_filtered_named_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_filtered_named_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetAllSubjects" => {
                    #[allow(non_camel_case_types)]
                    struct get_all_subjectsSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for get_all_subjectsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::EmptyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_all_subjects(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_all_subjectsSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetAllNamedSubjects" => {
                    #[allow(non_camel_case_types)]
                    struct get_all_named_subjectsSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::SimpleGetRequest> for get_all_named_subjectsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::SimpleGetRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_all_named_subjects(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_all_named_subjectsSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetAllObjects" => {
                    #[allow(non_camel_case_types)]
                    struct get_all_objectsSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for get_all_objectsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::EmptyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_all_objects(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_all_objectsSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetAllNamedObjects" => {
                    #[allow(non_camel_case_types)]
                    struct get_all_named_objectsSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::SimpleGetRequest> for get_all_named_objectsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::SimpleGetRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_all_named_objects(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_all_named_objectsSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetAllActions" => {
                    #[allow(non_camel_case_types)]
                    struct get_all_actionsSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for get_all_actionsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::EmptyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_all_actions(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_all_actionsSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetAllNamedActions" => {
                    #[allow(non_camel_case_types)]
                    struct get_all_named_actionsSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::SimpleGetRequest> for get_all_named_actionsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::SimpleGetRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_all_named_actions(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_all_named_actionsSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetAllRoles" => {
                    #[allow(non_camel_case_types)]
                    struct get_all_rolesSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for get_all_rolesSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::EmptyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_all_roles(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_all_rolesSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetAllNamedRoles" => {
                    #[allow(non_camel_case_types)]
                    struct get_all_named_rolesSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::SimpleGetRequest> for get_all_named_rolesSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::SimpleGetRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_all_named_roles(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_all_named_rolesSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/HasPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct has_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for has_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::has_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = has_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/HasNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct has_named_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for has_named_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::has_named_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = has_named_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/HasGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct has_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for has_grouping_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::has_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = has_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/HasNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct has_named_grouping_policySvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for has_named_grouping_policySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PolicyRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::has_named_grouping_policy(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = has_named_grouping_policySvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetRolesForUser" => {
                    #[allow(non_camel_case_types)]
                    struct get_roles_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for get_roles_for_userSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::UserRoleRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_roles_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_roles_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetImplicitRolesForUser" => {
                    #[allow(non_camel_case_types)]
                    struct get_implicit_roles_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for get_implicit_roles_for_userSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::UserRoleRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_implicit_roles_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_implicit_roles_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetUsersForRole" => {
                    #[allow(non_camel_case_types)]
                    struct get_users_for_roleSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for get_users_for_roleSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::UserRoleRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_users_for_role(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_users_for_roleSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/HasRoleForUser" => {
                    #[allow(non_camel_case_types)]
                    struct has_role_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for has_role_for_userSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::UserRoleRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::has_role_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = has_role_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/AddRoleForUser" => {
                    #[allow(non_camel_case_types)]
                    struct add_role_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for add_role_for_userSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::UserRoleRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::add_role_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = add_role_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/DeleteRoleForUser" => {
                    #[allow(non_camel_case_types)]
                    struct delete_role_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for delete_role_for_userSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::UserRoleRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::delete_role_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = delete_role_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/DeleteRolesForUser" => {
                    #[allow(non_camel_case_types)]
                    struct delete_roles_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for delete_roles_for_userSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::UserRoleRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::delete_roles_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = delete_roles_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/DeleteUser" => {
                    #[allow(non_camel_case_types)]
                    struct delete_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for delete_userSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::UserRoleRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::delete_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = delete_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/DeleteRole" => {
                    #[allow(non_camel_case_types)]
                    struct delete_roleSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for delete_roleSvc<T> {
                        type Response = super::EmptyReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::UserRoleRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::delete_role(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = delete_roleSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetPermissionsForUser" => {
                    #[allow(non_camel_case_types)]
                    struct get_permissions_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest> for get_permissions_for_userSvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PermissionRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_permissions_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_permissions_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/GetImplicitPermissionsForUser" => {
                    #[allow(non_camel_case_types)]
                    struct get_implicit_permissions_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest> for get_implicit_permissions_for_userSvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PermissionRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::get_implicit_permissions_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = get_implicit_permissions_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/DeletePermission" => {
                    #[allow(non_camel_case_types)]
                    struct delete_permissionSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest> for delete_permissionSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PermissionRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::delete_permission(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = delete_permissionSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/AddPermissionForUser" => {
                    #[allow(non_camel_case_types)]
                    struct add_permission_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest> for add_permission_for_userSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PermissionRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::add_permission_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = add_permission_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/DeletePermissionForUser" => {
                    #[allow(non_camel_case_types)]
                    struct delete_permission_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest> for delete_permission_for_userSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PermissionRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::delete_permission_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = delete_permission_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/DeletePermissionsForUser" => {
                    #[allow(non_camel_case_types)]
                    struct delete_permissions_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest> for delete_permissions_for_userSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PermissionRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::delete_permissions_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = delete_permissions_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/palm.casbin.v1.Casbin/HasPermissionForUser" => {
                    #[allow(non_camel_case_types)]
                    struct has_permission_for_userSvc<T: Casbin >(pub Arc<T>);

                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest> for has_permission_for_userSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::PermissionRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Casbin>::has_permission_for_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = has_permission_for_userSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }

                _ => Box::pin(async move {
                    let mut response = http::Response::new(tonic::body::Body::default());
                    let headers = response.headers_mut();
                    headers.insert(tonic::Status::GRPC_STATUS, (tonic::Code::Unimplemented as i32).into());
                    headers.insert(http::header::CONTENT_TYPE, tonic::metadata::GRPC_CONTENT_TYPE);
                    Ok(response)
                }),
            }
        }
    }

    impl<T> Clone for CasbinServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }

    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "palm.casbin.v1.Casbin";

    impl<T> tonic::server::NamedService for CasbinServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
