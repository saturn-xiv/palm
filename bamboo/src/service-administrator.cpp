#include "bamboo/services.hpp"

#define BAMBOO_JWT_ISSUER "bamboo"
#define BAMBOO_JWT_AUDIENCE "web"

grpc::Status bamboo::services::AdministratorServiceImpl::SignIn(
    grpc::ServerContext* context,
    const palm::router::v1::AdministratorSignInRequest* request,
    palm::router::v1::AdministratorSignInResponse* reply) {
  if (!bamboo::dao::administrator::auth(*this->_db, request->user().name(),
                                        request->user().password())) {
    return grpc::Status(grpc::StatusCode::UNAUTHENTICATED,
                        "invalid administrator account");
  }
  const auto token = this->_jwt->sign(
      BAMBOO_JWT_ISSUER, request->user().name(), {BAMBOO_JWT_AUDIENCE},
      std::nullopt, std::chrono::seconds{request->ttl().seconds()});
  const std::string en_us = "en-US";
  reply->set_token(token);
  reply->set_locale(en_us);
  reply->set_timezone("UTC");
  reply->add_languages(en_us);
  return grpc::Status::OK;
}

grpc::Status bamboo::services::AdministratorServiceImpl::SignOut(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    google::protobuf::Empty* reply) {
  const auto admin = bamboo::current_administrator(context, this->_jwt);
  if (!admin) {
    return grpc::Status(grpc::StatusCode::PERMISSION_DENIED, "");
  }
  spdlog::info("administrator({}) signed out", admin.value());
  return grpc::Status::OK;
}

grpc::Status bamboo::services::AdministratorServiceImpl::Update(
    grpc::ServerContext* context,
    const palm::router::v1::AdministratorUpdateRequest* request,
    google::protobuf::Empty* reply) {
  if (!bamboo::current_administrator(context, this->_jwt)) {
    return grpc::Status(grpc::StatusCode::PERMISSION_DENIED, "");
  }

  {
    soci::transaction tr(*this->_db);
    if (!bamboo::dao::administrator::auth(*this->_db, request->current().name(),
                                          request->current().password())) {
      return grpc::Status(grpc::StatusCode::UNAUTHENTICATED,
                          "invalid current administrator account");
    }
    spdlog::warn("update administrator {}=>{}", request->current().name(),
                 request->new_().name());
    bamboo::dao::administrator::save(*this->_db, request->new_().name(),
                                     request->new_().password());

    tr.commit();
  }
  return grpc::Status::OK;
}

std::optional<std::string> bamboo::current_administrator(
    grpc::ServerContext* context, std::shared_ptr<palm::Jwt> jwt) {
  palm::Session ss(context);
  const auto token = ss.token();
  if (!token) {
    return std::nullopt;
  }
  const auto& [jwt_id, key_id, subject, payload] =
      jwt->verify(token.value(), BAMBOO_JWT_ISSUER, BAMBOO_JWT_AUDIENCE);
  return subject;
}
