/// Generated client implementations.
pub mod jwt_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// --------------------------------------------------------
    #[derive(Debug, Clone)]
    pub struct JwtClient<T> {
        channel: T,
    }

    impl<T> JwtClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn sign<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::JwtSignResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::JwtSignRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.loquat.v1.Jwt/Sign", request)
        }

        pub fn verify<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::JwtVerifyResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::JwtVerifyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.loquat.v1.Jwt/Verify", request)
        }
    }
}
/// Generated client implementations.
pub mod h_mac_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// --------------------------------------------------------
    #[derive(Debug, Clone)]
    pub struct HMacClient<T> {
        channel: T,
    }

    impl<T> HMacClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn sign<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::HMacSignResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::HMacSignRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.loquat.v1.HMac/Sign", request)
        }

        pub fn verify<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::HMacVerifyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.loquat.v1.HMac/Verify", request)
        }
    }
}
/// Generated client implementations.
pub mod aes_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// --------------------------------------------------------
    #[derive(Debug, Clone)]
    pub struct AesClient<T> {
        channel: T,
    }

    impl<T> AesClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn encrypt<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::AesEncryptResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::AesEncryptRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.loquat.v1.Aes/Encrypt", request)
        }

        pub fn decrypt<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::AesDecryptResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::AesDecryptRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.loquat.v1.Aes/Decrypt", request)
        }
    }
}
/// Generated client implementations.
pub mod argon2_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// --------------------------------------------------------
    #[derive(Debug, Clone)]
    pub struct Argon2Client<T> {
        channel: T,
    }

    impl<T> Argon2Client<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        pub fn sign<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Argon2SignResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Argon2SignRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.loquat.v1.Argon2/Sign", request)
        }

        pub fn verify<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Argon2VerifyRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/palm.loquat.v1.Argon2/Verify", request)
        }
    }
}
