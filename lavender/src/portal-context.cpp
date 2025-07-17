
#include "palm/portal.hpp"

grpc::Status palm::portal::services::UserServiceImpl::SignInByEmail(
    grpc::ServerContext* context,
    const palm::portal::v1::UserSignInByEmailRequest* request,
    palm::portal::v1::UserSignInResponse* reply) {
  // TODO
  reply->set_token("ttt");
  return grpc::Status::OK;
}

std::shared_ptr<palm::portal::v1::UserSignInResponse>
palm::portal::rpc::UserClient::sign_in(const std::string& email,
                                       const std::string& password) {
  palm::portal::v1::UserSignInByEmailRequest request;
  //   TODO check email
  request.set_email(email);
  //   TODO check password
  request.set_password(password);

  std::shared_ptr<palm::portal::v1::UserSignInResponse> reply =
      std::make_shared<palm::portal::v1::UserSignInResponse>();

  grpc::ClientContext context;
  grpc::Status status =
      this->_stub->SignInByEmail(&context, request, reply.get());

  if (!status.ok()) {
    spdlog::error("{} {}", static_cast<int>(status.error_code()),
                  status.error_message());
    return nullptr;
  }
  return reply;
}
