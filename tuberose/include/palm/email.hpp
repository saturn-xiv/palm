#pragma once

#include "daisy.grpc.pb.h"
#include "palm/env.hpp"

#include <mailio/message.hpp>

namespace palm {

namespace smtp {

struct Address {
  Address() {}
  Address(const toml::table& root);

  std::string name;
  std::string email;
};

// https://developers.google.com/gmail/imap/imap-smtp
class Config {
 public:
  Config(const toml::table& root);
  Config(const std::string& host, const uint16_t port,
         const int32_t auth_method, const std::shared_ptr<Address> user,
         const std::string& password, std::vector<Address> cc = {},
         std::vector<Address> bcc = {})
      : _host(host),
        _port(port),
        _auth_method(auth_method),
        _user(user),
        _password(password),
        _cc(cc),
        _bcc(bcc) {}

  void send(const palm::daisy::v1::EmailTask* task) const;

 private:
  static std::pair<mailio::message::media_type_t, std::string> detect(
      const std::string& name);
  void send(mailio::message& msg) const;

  std::string _host;
  uint16_t _port;
  std::shared_ptr<Address> _user;
  std::string _password;
  std::vector<Address> _cc;
  std::vector<Address> _bcc;
  int32_t _auth_method;
};
}  // namespace smtp

}  // namespace palm
