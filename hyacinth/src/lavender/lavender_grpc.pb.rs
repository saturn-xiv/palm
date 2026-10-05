/// Generated client implementations.
pub mod storage_service_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    #[derive(Debug, Clone)]
    pub struct StorageServiceClient<T> {
        channel: T,
    }

    impl<T> StorageServiceClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn report<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ReportRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.lavender.v1.StorageService/Report", request)
        }
    }
}
