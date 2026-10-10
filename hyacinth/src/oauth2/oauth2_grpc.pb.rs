/// Generated client implementations.
pub mod google_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    #[derive(Debug, Clone)]
    pub struct GoogleClient<T> {
        channel: T,
    }

    impl<T> GoogleClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn sign_in_url<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::GoogleSignInUrlResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::GoogleSignInUrlRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.oauth2.v1.Google/SignInUrl", request)
        }

        pub fn fetch_token<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::GoogleFetchTokenResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::GoogleFetchTokenRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.oauth2.v1.Google/FetchToken", request)
        }

        pub fn user_info<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::GoogleUserInfoResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::google_fetch_token_response::Credentials> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.oauth2.v1.Google/UserInfo", request)
        }

        pub fn revoke<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::GoogleRevokeResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::google_fetch_token_response::Credentials> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.oauth2.v1.Google/Revoke", request)
        }
    }
}
