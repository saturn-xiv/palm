/// Generated client implementations.
pub mod storage_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    #[derive(Debug, Clone)]
    pub struct StorageClient<T> {
        channel: T,
    }

    impl<T> StorageClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn systemd<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::SystemdRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.lavender.v1.Storage/Systemd", request)
        }

        pub fn report<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ReportRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.lavender.v1.Storage/Report", request)
        }
    }
}
