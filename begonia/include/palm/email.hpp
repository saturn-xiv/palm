#pragma once

#include <cstdint>
#include <optional>
#include <string>
#include <utility>
#include <vector>

namespace palm {
namespace email {
struct Account {
  std::string name;
  std::string email;
};
struct Attachment {
  std::string name;
  std::pair<std::string, std::string> content_type;
  std::vector<uint8_t> body;
};
struct Body {
  bool html;
  std::string content;
};
class Smtp {
 public:
  Smtp(const std::string& host, uint16_t port, const Account& user,
       const std::string& password)
      : _host(host), _port(port), _user(user), _password(password) {}
  void send(const Account& to, const std::vector<Account> cc,
            const std::vector<Account> bcc, const std::string& subject,
            const Body& body, const std::vector<Attachment> attachments) const;

 private:
  std::string _host;
  uint16_t _port;
  Account _user;
  std::string _password;
};
}  // namespace email
}  // namespace palm
