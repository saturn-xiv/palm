/// Generated client implementations.
pub mod enforcer_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    #[derive(Debug, Clone)]
    pub struct EnforcerClient<T> {
        channel: T,
    }

    impl<T> EnforcerClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn get_all_subjects<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::SubjectsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Empty> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/GetAllSubjects", request)
        }

        pub fn get_all_objects<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ObjectsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Empty> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/GetAllObjects", request)
        }

        pub fn get_all_actions<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ActionsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Empty> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/GetAllActions", request)
        }

        pub fn get_all_roles<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::RolesResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Empty> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/GetAllRoles", request)
        }

        pub fn get_roles_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::RolesResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::subject::User> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/GetRolesForUser", request)
        }

        pub fn get_implicit_roles_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::RolesResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::subject::User> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/GetImplicitRolesForUser", request)
        }

        pub fn get_users_for_role<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::UsersResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::subject::Role> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/GetUsersForRole", request)
        }

        pub fn get_implicit_users_for_role<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::UsersResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::subject::Role> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/GetImplicitUsersForRole", request)
        }

        pub fn has_role_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/HasRoleForUser", request)
        }

        pub fn add_role_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/AddRoleForUser", request)
        }

        pub fn delete_role_for_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UserRoleRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/DeleteRoleForUser", request)
        }

        pub fn delete_user<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::subject::User> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/DeleteUser", request)
        }

        pub fn delete_role<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::subject::Role> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/DeleteRole", request)
        }

        pub fn get_permissions<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::PermissionsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Subject> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/GetPermissions", request)
        }

        pub fn get_implicit_permissions<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::PermissionsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Subject> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/GetImplicitPermissions", request)
        }

        pub fn delete_permission<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Permission> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/DeletePermission", request)
        }

        pub fn add_permission<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Permission> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/AddPermission", request)
        }

        pub fn has_permission<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Permission> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.rbac.v1.Enforcer/HasPermission", request)
        }
    }
}
