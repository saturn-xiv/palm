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

        pub fn systemd(&self) -> ClientStreamingCallBuilder<'_, &T, super::SystemdRequest, super::SystemdResponse> {
          ClientStreamingCallBuilder::new(&self.channel, "/palm.lavender.v1.Reporter/Systemd")
        }

        pub fn kubernetes(&self) -> ClientStreamingCallBuilder<'_, &T, super::KubernetesRequest, super::KubernetesResponse> {
          ClientStreamingCallBuilder::new(&self.channel, "/palm.lavender.v1.Reporter/Kubernetes")
        }

        pub fn claw<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ClawRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.lavender.v1.Reporter/Claw", request)
        }
    }
}
