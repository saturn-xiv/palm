#include "loquat/service.hpp"
#include "loquat/env.hpp"

grpc::Status loquat::JwtService::Sign(
    grpc::ServerContext* context,
    const palm::loquat::v1::JwtSignRequest* request,
    palm::loquat::v1::JwtSignResponse* response) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  loquat::Jwt jwt;

  const std::optional<std::string> jwt_id =
      request->has_jwt_id() ? std::optional<std::string>{request->jwt_id()}
                            : std::nullopt;
  const std::optional<std::string> key_id =
      request->has_key_id() ? std::optional<std::string>{request->key_id()}
                            : std::nullopt;
  const std::optional<std::string> payload =
      request->has_payload() ? std::optional<std::string>{request->payload()}
                             : std::nullopt;

  std::set<std::string> audiences;
  for (const auto& it : request->audiences()) {
    audiences.insert(it);
  }
  const auto token =
      jwt.sign(jwt_id, key_id, request->issuer(), request->subject(), audiences,
               absl::FromUnixSeconds(request->issued_at()),
               absl::FromUnixSeconds(request->not_before()),
               absl::FromUnixSeconds(request->expired_at()), payload);
  if (token) {
    response->set_token(token.value());
    return grpc::Status::OK;
  }
  return grpc::Status(grpc::StatusCode::INTERNAL, "Jwt sign failed");
}
grpc::Status loquat::JwtService::Verify(
    grpc::ServerContext* context,
    const palm::loquat::v1::JwtVerifyRequest* request,
    palm::loquat::v1::JwtVerifyResponse* response) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  loquat::Jwt jwt;
  const auto it =
      jwt.verify(request->token(), request->issuer(), request->audience());
  if (it) {
    response->set_subject(it->first);
    if (it->second) {
      response->set_payload(it->second.value());
    }
    return grpc::Status::OK;
  }
  return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT,
                      "JWT verification check failed");
}
grpc::Status loquat::HMacService::Sign(
    grpc::ServerContext* context,
    const palm::loquat::v1::HMacSignRequest* request,
    palm::loquat::v1::HMacSignResponse* response) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  loquat::HMac mac;
  const auto hashed = mac.sign(request->plain());
  if (hashed) {
    response->set_hashed(hashed.value());
    return grpc::Status::OK;
  }
  return grpc::Status(grpc::StatusCode::INTERNAL, "HMac sign failed");
}
grpc::Status loquat::HMacService::Verify(
    grpc::ServerContext* context,
    const palm::loquat::v1::HMacVerifyRequest* request,
    palm::loquat::v1::Empty* response) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  loquat::HMac mac;
  return mac.verify(request->hashed(), request->plain())
             ? grpc::Status::OK
             : grpc::Status(grpc::StatusCode::INVALID_ARGUMENT,
                            "HMac verification check failed");
  ;
}
grpc::Status loquat::AesService::Encrypt(
    grpc::ServerContext* context,
    const palm::loquat::v1::AesEncryptRequest* request,
    palm::loquat::v1::AesEncryptResponse* response) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  loquat::Aes aes;
  const auto cipher = aes.encrypt(request->plain(), request->associated_data());
  if (cipher) {
    response->set_cipher(cipher.value());
    return grpc::Status::OK;
  }
  return grpc::Status(grpc::StatusCode::INTERNAL, "AES encrypt failed");
}
grpc::Status loquat::AesService::Decrypt(
    grpc::ServerContext* context,
    const palm::loquat::v1::AesDecryptRequest* request,
    palm::loquat::v1::AesDecryptResponse* response) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  loquat::Aes aes;
  const auto plain = aes.decrypt(request->cipher(), request->associated_data());
  if (plain) {
    response->set_plain(plain.value());
    return grpc::Status::OK;
  }
  return grpc::Status(grpc::StatusCode::INTERNAL, "AES decrypt failed");
}
grpc::Status loquat::Argon2Service::Sign(
    grpc::ServerContext* context,
    const palm::loquat::v1::Argon2SignRequest* request,
    palm::loquat::v1::Argon2SignResponse* response) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  const auto it = loquat::argon2id::sign(request->password());
  if (it) {
    response->set_hashed(it.value());
    return grpc::Status::OK;
  }
  return grpc::Status(grpc::StatusCode::INTERNAL, "sign check failed");
}
grpc::Status loquat::Argon2Service::Verify(
    grpc::ServerContext* context,
    const palm::loquat::v1::Argon2VerifyRequest* request,
    palm::loquat::v1::Empty* response) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);

  return loquat::argon2id::verify(request->hashed(), request->password())
             ? grpc::Status::OK
             : grpc::Status(grpc::StatusCode::INVALID_ARGUMENT,
                            "verification check failed");
}
