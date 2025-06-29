#pragma once

#include "basil/cache.hpp"
#include "basil/crypto.hpp"
#include "basil/email.hpp"
#include "basil/orm.hpp"
#include "basil/queue.hpp"
#include "basil/s3.hpp"
#include "basil/search.hpp"
#include "basil/session.hpp"
#include "basil/theme.hpp"
#include "basil/twilio.hpp"
#include "basil/validator.hpp"
#include "wisteria.grpc.pb.h"

#include <format>

namespace basil {
class GrpcClient {
 public:
  GrpcClient(const std::string& host, uint16_t port)
      : _host(host), _port(port) {}

  inline std::string target() {
    return std::format("{}:{}", this->_host, this->_port);
  }

 private:
  std::string _host;
  uint16_t _port;
};

namespace wisteria {

void mount(httplib::Server& server, basil::GrpcClient& rpc, basil::Theme& theme,
           std::shared_ptr<basil::Jwt> jwt, std::shared_ptr<basil::Minio> s3);

namespace workers {
class SmsSendQueueConsumer : public basil::QueueConsumer {
 public:
  SmsSendQueueConsumer(const std::string& name,
                       std::shared_ptr<basil::Twilio> twilio)
      : _name(name), _twilio(twilio) {}
  std::string name() override { return _name; }
  void execute(const std::string& id, const std::string& content_type,
               const std::vector<uint8_t> payload) override;

  inline static const std::string QUEUE = "sms.send";

 private:
  std::shared_ptr<basil::Twilio> _twilio;
  std::string _name;
};

class EmailSendQueueConsumer : public basil::QueueConsumer {
 public:
  EmailSendQueueConsumer(const std::string& name,
                         std::shared_ptr<basil::email::Smtp> smtp)
      : _name(name), _smtp(smtp) {}

  std::string name() override { return _name; }
  void execute(const std::string& id, const std::string& content_type,
               const std::vector<uint8_t> payload) override;

  inline static const std::string QUEUE = "email.send";

 private:
  std::shared_ptr<basil::email::Smtp> _smtp;
  std::string _name;
};
}  // namespace workers

namespace services {
class UserServiceImpl final : public basil::wisteria::v1::User::Service {
 public:
  UserServiceImpl(std::shared_ptr<sw::redis::Redis> cache,
                  std::shared_ptr<basil::rabbitmq::Config> queue,
                  std::shared_ptr<basil::Minio> s3,
                  std::shared_ptr<basil::Aes> aes,
                  std::shared_ptr<basil::HMac> hmac,
                  std::shared_ptr<basil::Jwt> jwt)
      : _cache(cache),
        _queue(queue),
        _s3(s3),
        _aes(aes),
        _hmac(hmac),
        _jwt(jwt) {}

  grpc::Status SignInByEmail(
      grpc::ServerContext* context,
      const basil::wisteria::v1::UserSignInByEmailRequest* request,
      basil::wisteria::v1::UserSignInResponse* reply) override;

 private:
  std::shared_ptr<basil::rabbitmq::Config> _queue;
  std::shared_ptr<sw::redis::Redis> _cache;
  std::shared_ptr<basil::Minio> _s3;
  std::shared_ptr<basil::Aes> _aes;
  std::shared_ptr<basil::Jwt> _jwt;
  std::shared_ptr<basil::HMac> _hmac;
};
class PolicyServiceImpl final : public basil::wisteria::v1::Policy::Service {};
class SiteServiceImpl final : public basil::wisteria::v1::Site::Service {
 public:
  SiteServiceImpl(std::shared_ptr<basil::opensearch::Client> search)
      : _search(search) {}

 private:
  std::shared_ptr<basil::opensearch::Client> _search;
};
}  // namespace services
namespace rpc {

class UserClient {
 public:
  UserClient(std::shared_ptr<grpc::Channel> channel)
      : _stub(basil::wisteria::v1::User::NewStub(channel)) {}

  std::shared_ptr<basil::wisteria::v1::UserSignInResponse> sign_in(
      const std::string& email, const std::string& password);

 private:
  std::unique_ptr<basil::wisteria::v1::User::Stub> _stub;
};
}  // namespace rpc
}  // namespace wisteria
}  // namespace basil
