#pragma once

#include "loquat/env.hpp"
#include "loquat.grpc.pb.h"

namespace loquat {



class JwtService final : public palm::loquat::v1::Jwt::Service {
  grpc::Status Sign(grpc::ServerContext* context,
                    const palm::loquat::v1::JwtSignRequest* request,
                    palm::loquat::v1::JwtSignResponse* response) override;
  grpc::Status Verify(grpc::ServerContext* context,
                      const palm::loquat::v1::JwtVerifyRequest* request,
                      palm::loquat::v1::JwtVerifyResponse* response) override;
};

class HMacService final : public palm::loquat::v1::HMac::Service {
  grpc::Status Sign(grpc::ServerContext* context,
                    const palm::loquat::v1::HMacSignRequest* request,
                    palm::loquat::v1::HMacSignResponse* response) override;
  grpc::Status Verify(grpc::ServerContext* context,
                      const palm::loquat::v1::HMacVerifyRequest* request,
                      palm::loquat::v1::Empty* response) override;
};

class AesService final : public palm::loquat::v1::Aes::Service {
  grpc::Status Encrypt(grpc::ServerContext* context,
                       const palm::loquat::v1::AesEncryptRequest* request,
                       palm::loquat::v1::AesEncryptResponse* response) override;
  grpc::Status Decrypt(grpc::ServerContext* context,
                       const palm::loquat::v1::AesDecryptRequest* request,
                       palm::loquat::v1::AesDecryptResponse* response) override;
};

class Argon2Service final : public palm::loquat::v1::Argon2::Service {
  grpc::Status Sign(grpc::ServerContext* context,
                    const palm::loquat::v1::Argon2SignRequest* request,
                    palm::loquat::v1::Argon2SignResponse* response) override;
  grpc::Status Verify(grpc::ServerContext* context,
                      const palm::loquat::v1::Argon2VerifyRequest* request,
                      palm::loquat::v1::Empty* response) override;
};

}  // namespace loquat
