
#include "basil/wisteria.hpp"

#include <boost/current_function.hpp>

grpc::Status basil::wisteria::services::UserServiceImpl::SignInByEmail(
    grpc::ServerContext* context,
    const basil::wisteria::v1::UserSignInByEmailRequest* request,
    basil::wisteria::v1::UserSignInResponse* reply) {
  // TODO
  reply->set_token("ttt");
  return grpc::Status::OK;
}

std::shared_ptr<basil::wisteria::v1::UserSignInResponse>
basil::wisteria::rpc::UserClient::sign_in(const std::string& email,
                                          const std::string& password) {
  basil::wisteria::v1::UserSignInByEmailRequest request;
  //   TODO check email
  request.set_email(email);
  //   TODO check password
  request.set_password(password);

  std::shared_ptr<basil::wisteria::v1::UserSignInResponse> reply =
      std::make_shared<basil::wisteria::v1::UserSignInResponse>();

  grpc::ClientContext context;
  grpc::Status status =
      this->_stub->SignInByEmail(&context, request, reply.get());

  if (!status.ok()) {
    BOOST_LOG_TRIVIAL(error)
        << status.error_code() << ": " << status.error_message();
    return nullptr;
  }
  return reply;
}

void basil::wisteria::workers::EmailSendQueueConsumer::execute(
    const std::string& id, const std::string& content_type,
    const std::vector<uint8_t> payload) {
  // TODO
}

void basil::wisteria::workers::SmsSendQueueConsumer::execute(
    const std::string& id, const std::string& content_type,
    const std::vector<uint8_t> payload) {
  // TODO
}

void basil::wisteria::mount(httplib::Server& server, basil::GrpcClient& rpc,
                            basil::Theme& theme,
                            std::shared_ptr<basil::Jwt> jwt,
                            std::shared_ptr<basil::Minio> s3) {
  BOOST_LOG_TRIVIAL(debug) << BOOST_CURRENT_FUNCTION;
  server.Get("/", [&](const auto& req, auto& res) {
    nlohmann::json data;
    data["title"] = "hi";
    const auto body = theme.render("home.html", data);
    res.set_content(body, basil::http::content_type::TEXT_HTML_UTF8);
  });
}
