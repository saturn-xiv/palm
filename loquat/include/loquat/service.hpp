#pragma once

#include "loquat.grpc.pb.h"
#include "loquat/env.hpp"

namespace loquat {

class JwtService final : public palm::loquat::v1::JwtService::Service {
  grpc::Status Compute(grpc::ServerContext* context,
                       const palm::loquat::v1::JwtComputeRequest* request,
                       palm::loquat::v1::JwtComputeResponse* response) override;
  grpc::Status Verify(grpc::ServerContext* context,
                      const palm::loquat::v1::JwtVerifyRequest* request,
                      palm::loquat::v1::JwtVerifyResponse* response) override;
};

class MacService final : public palm::loquat::v1::MacService::Service {
  grpc::Status Compute(grpc::ServerContext* context,
                       const palm::loquat::v1::MacComputeRequest* request,
                       palm::loquat::v1::MacComputeResponse* response) override;
  grpc::Status Verify(grpc::ServerContext* context,
                      const palm::loquat::v1::MacVerifyRequest* request,
                      google::protobuf::Empty* response) override;
};

class AeadService final : public palm::loquat::v1::AeadService::Service {
  grpc::Status Encrypt(
      grpc::ServerContext* context,
      const palm::loquat::v1::AeadEncryptRequest* request,
      palm::loquat::v1::AeadEncryptResponse* response) override;
  grpc::Status Decrypt(
      grpc::ServerContext* context,
      const palm::loquat::v1::AeadDecryptRequest* request,
      palm::loquat::v1::AeadDecryptResponse* response) override;
};

class Argon2Service final : public palm::loquat::v1::Argon2Service::Service {
  grpc::Status Compute(
      grpc::ServerContext* context,
      const palm::loquat::v1::Argon2ComputeRequest* request,
      palm::loquat::v1::Argon2ComputeResponse* response) override;
  grpc::Status Verify(grpc::ServerContext* context,
                      const palm::loquat::v1::Argon2VerifyRequest* request,
                      google::protobuf::Empty* response) override;
};

}  // namespace loquat
