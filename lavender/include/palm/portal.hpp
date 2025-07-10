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
  GrpcClient(const toml::table& config)
      : _host(config["host"].value<std::string>().value()),
        _port(config["port"].value<uint16_t>().value()) {}
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

namespace users {
struct Item {
  int id;
  std::string uid;
  std::string lang;
  std::string timezone;
  int sign_in_count;
  boost::optional<std::tm> current_sign_in_at;
  boost::optional<std::string> current_sign_in_ip;
  boost::optional<std::tm> last_sign_in_at;
  boost::optional<std::string> last_sign_in_ip;
  boost::optional<std::tm> locked_at;
  boost::optional<std::tm> deleted_at;
  int version;
  std::tm updated_at;
};
boost::optional<Item> get(soci::session& db, int id);
boost::optional<Item> get(soci::session& db, const std::string& uid);
void enable(soci::session& db, int id);
void disable(soci::session& db, int id);
namespace email {
struct Item {
  int id;
  int user_id;
  std::string real_name;
  std::string email;
  std::string password;
  std::string avatar;
  boost::optional<std::tm> confirmed_at;
  boost::optional<std::tm> deleted_at;
  int version;
  std::tm updated_at;
};
boost::optional<Item> get(soci::session& db, int id);
boost::optional<Item> get(soci::session& db, const std::string& email);
void enable(soci::session& db, int id);
void disable(soci::session& db, int id);
}  // namespace email
namespace wechat {
namespace mini_program {
struct Item {
  int id;
  int user_id;
  std::string union_id;
  std::string app_id;
  std::string open_id;
  boost::optional<std::string> nickname;
  boost::optional<std::string> avatar_url;
  boost::optional<std::tm> deleted_at;
  int version;
  std::tm updated_at;
};
boost::optional<Item> get(soci::session& db, int id);
void enable(soci::session& db, int id);
void disable(soci::session& db, int id);
boost::optional<Item> get(soci::session& db, const std::string& union_id);
boost::optional<Item> get(soci::session& db, const std::string& app_id,
                          const std::string& open_id);
}  // namespace mini_program
namespace oauth2 {
struct Item {
  int id;
  int user_id;
  std::string union_id;
  std::string app_id;
  std::string open_id;
  std::string nickname;
  int sex;
  std::string city;
  std::string province;
  std::string country;
  boost::optional<std::string> head_img_url;
  std::string privilege;
  std::string lang;
  boost::optional<std::tm> deleted_at;
  int version;
  std::tm updated_at;
};
boost::optional<Item> get(soci::session& db, int id);
boost::optional<Item> get(soci::session& db, const std::string& union_id);
boost::optional<Item> get(soci::session& db, const std::string& app_id,
                          const std::string& open_id);
void enable(soci::session& db, int id);
void disable(soci::session& db, int id);
}  // namespace oauth2
}  // namespace wechat
namespace google {
namespace oauth2 {
struct Item {
  int id;
  int user_id;
  std::string subject;
  boost::optional<std::string> email;
  int email_verified;
  boost::optional<std::string> name;
  boost::optional<std::string> picture;
  boost::optional<std::string> locale;
  boost::optional<std::tm> deleted_at;
  int version;
  std::tm updated_at;
};
boost::optional<Item> get(soci::session& db, int id);
void enable(soci::session& db, int id);
void disable(soci::session& db, int id);
}  // namespace oauth2
}  // namespace google
}  // namespace users

namespace locales {
struct Item {
  int id;
  std::string lang;
  std::string code;
  std::string message;
  std::tm updated_at;
};
std::vector<std::string> languages(soci::session& db);
void create(soci::session& db, const std::string& lang, const std::string& code,
            const std::string& message);
void update(soci::session& db, int id, const std::string& message);
void destroy(soci::session& db, int id);
int count(soci::session& db);
boost::optional<Item> get(soci::session& db, int id);
boost::optional<Item> get(soci::session& db, const std::string& lang,
                          const std::string& code);
boost::fusion::vector<Item> index(soci::session& db, int offset, int limit);
boost::fusion::vector<Item> by_lang(soci::session& db, const std::string& lang);
void load(soci::session& db, const std::filesystem::path& folder);
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

namespace soci {
template <>
struct type_conversion<palm::portal::dao::locales::Item> {
  typedef soci::values base_type;

  static void from_base(soci::values const& v, soci::indicator /* ind */,
                        palm::portal::dao::locales::Item& p) {
    p.id = v.get<int>("id");
    p.lang = v.get<std::string>("lang");
    p.code = v.get<std::string>("code");
    p.message = v.get<std::string>("message");
    p.updated_at = v.get<std::tm>("updated_at");
  }

  static void to_base(const palm::portal::dao::locales::Item& p,
                      soci::values& v, soci::indicator& ind) {
    v.set("id", p.id);
    v.set("lang", p.lang);
    v.set("code", p.code);
    v.set("message", p.message);
    v.set("updated_at", p.updated_at);
    ind = i_ok;
  }
};

template <>
struct type_conversion<palm::portal::dao::users::Item> {
  typedef soci::values base_type;

  static void from_base(soci::values const& v, soci::indicator /* ind */,
                        palm::portal::dao::users::Item& p) {
    p.id = v.get<int>("id");
    p.uid = v.get<std::string>("uid");
    p.lang = v.get<std::string>("lang");
    p.timezone = v.get<std::string>("timezone");
    p.sign_in_count = v.get<int>("sign_in_count");
    p.current_sign_in_at =
        v.get<boost::optional<std::tm>>("current_sign_in_at");
    p.current_sign_in_ip =
        v.get<boost::optional<std::string>>("current_sign_in_ip");
    p.last_sign_in_at = v.get<boost::optional<std::tm>>("last_sign_in_at");
    p.last_sign_in_ip = v.get<boost::optional<std::string>>("last_sign_in_ip");
    p.locked_at = v.get<boost::optional<std::tm>>("locked_at");
    p.deleted_at = v.get<boost::optional<std::tm>>("deleted_at");
    p.version = v.get<int>("version");
    p.updated_at = v.get<std::tm>("updated_at");
  }

  static void to_base(const palm::portal::dao::users::Item& p, soci::values& v,
                      soci::indicator& ind) {
    v.set("id", p.id);
    v.set("uid", p.uid);
    v.set("lang", p.lang);
    v.set("timezone", p.timezone);
    v.set("sign_in_count", p.sign_in_count);
    v.set("current_sign_in_at", p.current_sign_in_at);
    v.set("current_sign_in_ip", p.current_sign_in_ip);
    v.set("last_sign_in_at", p.last_sign_in_at);
    v.set("last_sign_in_ip", p.last_sign_in_ip);
    v.set("locked_at", p.locked_at);
    v.set("deleted_at", p.deleted_at);
    v.set("version", p.version);
    v.set("updated_at", p.updated_at);
    ind = i_ok;
  }
};

template <>
struct type_conversion<palm::portal::dao::users::email::Item> {
  typedef soci::values base_type;

  static void from_base(soci::values const& v, soci::indicator /* ind */,
                        palm::portal::dao::users::email::Item& p) {
    p.id = v.get<int>("id");
    p.user_id = v.get<int>("user_id");
    p.real_name = v.get<std::string>("real_name");
    p.email = v.get<std::string>("email");
    p.password = v.get<std::string>("password");
    p.avatar = v.get<std::string>("avatar");
    p.confirmed_at = v.get<boost::optional<std::tm>>("confirmed_at");
    p.deleted_at = v.get<boost::optional<std::tm>>("deleted_at");
    p.version = v.get<int>("version");
    p.updated_at = v.get<std::tm>("updated_at");
  }

  static void to_base(const palm::portal::dao::users::email::Item& p,
                      soci::values& v, soci::indicator& ind) {
    v.set("id", p.id);
    v.set("user_id", p.user_id);
    v.set("real_name", p.real_name);
    v.set("email", p.email);
    v.set("password", p.password);
    v.set("avatar", p.avatar);
    v.set("confirmed_at", p.confirmed_at);
    v.set("deleted_at", p.deleted_at);
    v.set("version", p.version);
    v.set("updated_at", p.updated_at);
    ind = i_ok;
  }
};

template <>
struct type_conversion<palm::portal::dao::users::wechat::mini_program::Item> {
  typedef soci::values base_type;

  static void from_base(
      soci::values const& v, soci::indicator /* ind */,
      palm::portal::dao::users::wechat::mini_program::Item& p) {
    p.id = v.get<int>("id");
    p.user_id = v.get<int>("user_id");
    p.union_id = v.get<std::string>("union_id");
    p.app_id = v.get<std::string>("app_id");
    p.open_id = v.get<std::string>("open_id");
    p.nickname = v.get<boost::optional<std::string>>("nickname");
    p.avatar_url = v.get<boost::optional<std::string>>("avatar_url");
    p.deleted_at = v.get<boost::optional<std::tm>>("deleted_at");
    p.version = v.get<int>("version");
    p.updated_at = v.get<std::tm>("updated_at");
  }

  static void to_base(
      const palm::portal::dao::users::wechat::mini_program::Item& p,
      soci::values& v, soci::indicator& ind) {
    v.set("id", p.id);
    v.set("user_id", p.user_id);
    v.set("union_id", p.union_id);
    v.set("app_id", p.app_id);
    v.set("open_id", p.open_id);
    v.set("nickname", p.nickname);
    v.set("avatar_url", p.avatar_url);
    v.set("deleted_at", p.deleted_at);
    v.set("version", p.version);
    v.set("updated_at", p.updated_at);
    ind = i_ok;
  }
};

template <>
struct type_conversion<palm::portal::dao::users::wechat::oauth2::Item> {
  typedef soci::values base_type;

  static void from_base(soci::values const& v, soci::indicator /* ind */,
                        palm::portal::dao::users::wechat::oauth2::Item& p) {
    p.id = v.get<int>("id");
    p.user_id = v.get<int>("user_id");
    p.union_id = v.get<std::string>("union_id");
    p.app_id = v.get<std::string>("app_id");
    p.open_id = v.get<std::string>("open_id");
    p.nickname = v.get<std::string>("nickname");
    p.sex = v.get<int>("sex");
    p.city = v.get<std::string>("city");
    p.province = v.get<std::string>("province");
    p.country = v.get<std::string>("country");
    p.head_img_url = v.get<boost::optional<std::string>>("head_img_url");
    p.lang = v.get<std::string>("lang");
    p.privilege = v.get<std::string>("privilege");
    p.deleted_at = v.get<boost::optional<std::tm>>("deleted_at");
    p.version = v.get<int>("version");
    p.updated_at = v.get<std::tm>("updated_at");
  }

  static void to_base(const palm::portal::dao::users::wechat::oauth2::Item& p,
                      soci::values& v, soci::indicator& ind) {
    v.set("id", p.id);
    v.set("user_id", p.user_id);
    v.set("union_id", p.union_id);
    v.set("app_id", p.app_id);
    v.set("open_id", p.open_id);
    v.set("nickname", p.nickname);
    v.set("sex", p.sex);
    v.set("city", p.city);
    v.set("province", p.province);
    v.set("country", p.country);
    v.set("head_img_url", p.head_img_url);
    v.set("privilege", p.privilege);
    v.set("lang", p.lang);
    v.set("deleted_at", p.deleted_at);
    v.set("version", p.version);
    v.set("updated_at", p.updated_at);
    ind = i_ok;
  }
};

template <>
struct type_conversion<palm::portal::dao::users::google::oauth2::Item> {
  typedef soci::values base_type;

  static void from_base(soci::values const& v, soci::indicator /* ind */,
                        palm::portal::dao::users::google::oauth2::Item& p) {
    p.id = v.get<int>("id");
    p.user_id = v.get<int>("user_id");
    p.subject = v.get<std::string>("subject");
    p.email = v.get<boost::optional<std::string>>("email");
    p.email_verified = v.get<int>("email_verified");
    p.name = v.get<boost::optional<std::string>>("name");
    p.picture = v.get<boost::optional<std::string>>("picture");
    p.locale = v.get<boost::optional<std::string>>("locale");
    p.deleted_at = v.get<boost::optional<std::tm>>("deleted_at");
    p.version = v.get<int>("version");
    p.updated_at = v.get<std::tm>("updated_at");
  }

  static void to_base(const palm::portal::dao::users::google::oauth2::Item& p,
                      soci::values& v, soci::indicator& ind) {
    v.set("id", p.id);
    v.set("user_id", p.user_id);
    v.set("subject", p.subject);
    v.set("email", p.email);
    v.set("email_verified", p.email_verified);
    v.set("name", p.name);
    v.set("picture", p.picture);
    v.set("locale", p.locale);
    v.set("deleted_at", p.deleted_at);
    v.set("version", p.version);
    v.set("updated_at", p.updated_at);
    ind = i_ok;
  }
};
}  // namespace soci
