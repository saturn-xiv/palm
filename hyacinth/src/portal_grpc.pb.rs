/// Generated client implementations.
pub mod user_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    #[derive(Debug, Clone)]
    pub struct UserClient<T> {
        channel: T,
    }

    impl<T> UserClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

    }
}
/// Generated client implementations.
pub mod site_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    #[derive(Debug, Clone)]
    pub struct SiteClient<T> {
        channel: T,
    }

    impl<T> SiteClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn heartbeat<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::SiteHeartbeatResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Empty> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.portal.v1.Site/Heartbeat", request)
        }
    }
}
