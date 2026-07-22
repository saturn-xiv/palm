/// Generated client implementations.
pub mod wechat_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    #[derive(Debug, Clone)]
    pub struct WechatClient<T> {
        channel: T,
    }

    impl<T> WechatClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn bill_pull<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::BillPullResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::BillPullRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.wechatpay.v1.Wechat/BillPull", request)
        }
    }
}
