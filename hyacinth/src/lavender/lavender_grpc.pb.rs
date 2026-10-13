/// Generated client implementations.
pub mod stash_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    #[derive(Debug, Clone)]
    pub struct StashClient<T> {
        channel: T,
    }

    impl<T> StashClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn systemd(&self) -> ClientStreamingCallBuilder<'_, &T, super::SystemdRequest, super::SystemdResponse> {
          ClientStreamingCallBuilder::new(&self.channel, "/palm.lavender.v1.Stash/Systemd")
        }

        pub fn kubernetes(&self) -> ClientStreamingCallBuilder<'_, &T, super::KubernetesRequest, super::KubernetesResponse> {
          ClientStreamingCallBuilder::new(&self.channel, "/palm.lavender.v1.Stash/Kubernetes")
        }

        pub fn claw<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ClawRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.lavender.v1.Stash/Claw", request)
        }
    }
}
