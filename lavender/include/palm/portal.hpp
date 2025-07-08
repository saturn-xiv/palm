#pragma once

#include "palm/cache.hpp"
#include "palm/crypto.hpp"
#include "palm/email.hpp"
#include "palm/orm.hpp"
#include "palm/queue.hpp"
#include "palm/s3.hpp"
#include "palm/search.hpp"
#include "palm/session.hpp"
#include "palm/theme.hpp"
#include "palm/twilio.hpp"
#include "palm/validator.hpp"
#include "portal.grpc.pb.h"

#include <format>

namespace palm {
class GrpcClient {
 public:
  GrpcClient(toml::table* config)
      : _host(config->get("host")->value<std::string>().value()),
        _port(config->get("port")->value<uint16_t>().value()) {}
  GrpcClient(const std::string& host, uint16_t port)
      : _host(host), _port(port) {}

  inline std::string target() {
    return std::format("{}:{}", this->_host, this->_port);
  }

 private:
  std::string _host;
  uint16_t _port;
};

namespace portal {

namespace dao {

std::pair<uint32_t, uint32_t> paginate(palm::portal::v1::Page* page,
                                       palm::portal::v1::Pagination* pagination,
                                       uint32_t total);
namespace locales {
struct Item {
  uint32_t id;
  std::string lang;
  std::string code;
  std::string message;
  std::tm updated_at;
};
std::vector<std::string> languages(soci::session& db);
void create(soci::session& db, const std::string& lang, const std::string& code,
            const std::string& message);
void update(soci::session& db, uint32_t id, const std::string& message);
uint32_t count(soci::session& db);
std::vector<Item> index(soci::session& db, uint32_t offset, uint32_t limit);
std::vector<Item> by_lang(soci::session& db, const std::string& lang);
}  // namespace locales
}  // namespace dao

void mount(httplib::Server& server, palm::GrpcClient& rpc, palm::Theme& theme,
           std::shared_ptr<palm::Jwt> jwt, std::shared_ptr<palm::Minio> s3);

namespace workers {
class SmsSendQueueConsumer : public palm::QueueConsumer {
 public:
  SmsSendQueueConsumer(const std::string& name,
                       std::shared_ptr<palm::Twilio> twilio)
      : _name(name), _twilio(twilio) {}
  std::string name() override { return _name; }
  void execute(const std::string& id, const std::string& content_type,
               const std::vector<uint8_t> payload) override;

  inline static const std::string QUEUE = "sms.send";

 private:
  std::shared_ptr<palm::Twilio> _twilio;
  std::string _name;
};

class EmailSendQueueConsumer : public palm::QueueConsumer {
 public:
  EmailSendQueueConsumer(const std::string& name,
                         std::shared_ptr<palm::email::Smtp> smtp)
      : _name(name), _smtp(smtp) {}

  std::string name() override { return _name; }
  void execute(const std::string& id, const std::string& content_type,
               const std::vector<uint8_t> payload) override;

  inline static const std::string QUEUE = "email.send";

 private:
  std::shared_ptr<palm::email::Smtp> _smtp;
  std::string _name;
};
}  // namespace workers

namespace services {
class UserServiceImpl final : public palm::portal::v1::User::Service {
 public:
  UserServiceImpl(std::shared_ptr<sw::redis::Redis> cache,
                  std::shared_ptr<palm::rabbitmq::Config> queue,
                  std::shared_ptr<palm::Minio> s3,
                  std::shared_ptr<palm::Aes> aes,
                  std::shared_ptr<palm::HMac> hmac,
                  std::shared_ptr<palm::Jwt> jwt)
      : _cache(cache),
        _queue(queue),
        _s3(s3),
        _aes(aes),
        _hmac(hmac),
        _jwt(jwt) {}

  grpc::Status SignInByEmail(
      grpc::ServerContext* context,
      const palm::portal::v1::UserSignInByEmailRequest* request,
      palm::portal::v1::UserSignInResponse* reply) override;

 private:
  std::shared_ptr<palm::rabbitmq::Config> _queue;
  std::shared_ptr<sw::redis::Redis> _cache;
  std::shared_ptr<palm::Minio> _s3;
  std::shared_ptr<palm::Aes> _aes;
  std::shared_ptr<palm::Jwt> _jwt;
  std::shared_ptr<palm::HMac> _hmac;
};
class PolicyServiceImpl final : public palm::portal::v1::Policy::Service {};
class SiteServiceImpl final : public palm::portal::v1::Site::Service {
 public:
  SiteServiceImpl(std::shared_ptr<palm::opensearch::Client> search)
      : _search(search) {}

 private:
  std::shared_ptr<palm::opensearch::Client> _search;
};
}  // namespace services
namespace rpc {

class UserClient {
 public:
  UserClient(std::shared_ptr<grpc::Channel> channel)
      : _stub(palm::portal::v1::User::NewStub(channel)) {}

  std::shared_ptr<palm::portal::v1::UserSignInResponse> sign_in(
      const std::string& email, const std::string& password);

 private:
  std::unique_ptr<palm::portal::v1::User::Stub> _stub;
};
}  // namespace rpc
}  // namespace portal
}  // namespace palm
