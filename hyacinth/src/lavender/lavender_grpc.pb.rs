/// Generated client implementations.
pub mod reporter_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    #[derive(Debug, Clone)]
    pub struct ReporterClient<T> {
        channel: T,
    }

    impl<T> ReporterClient<T>
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
          UnaryCallBuilder::new(&self.channel, "/palm.lavender.v1.Reporter/Systemd", request)
        }
    }
}
