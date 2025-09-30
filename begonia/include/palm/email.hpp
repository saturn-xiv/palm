#pragma once

#include <cstdint>
#include <filesystem>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace palm {
namespace email {
struct Account {
  std::string name;
  std::string email;

  Account() {}
  Account(const toml::table& config)
      : name(config["name"].value<std::string>().value()),
        email(config["email"].value<std::string>().value()) {}
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
  Smtp(const toml::table& config)
      : _host(config["host"].value<std::string>().value()),
        _port(config["port"].value<uint16_t>().value_or(465)),
        _user(*(config["user"].as_table())),
        _password(config["password"].value<std::string>().value()) {}
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

class Imap {
 public:
  Imap(const toml::table& config)
      : _host(config["host"].value<std::string>().value()),
        _port(config["port"].value<uint16_t>().value_or(465)),
        _user(*(config["user"].as_table())),
        _password(config["password"].value<std::string>().value()) {}
  Imap(const std::string& host, uint16_t port, const Account& user,
       const std::string& password)
      : _host(host), _port(port), _user(user), _password(password) {}
  void fetch(const std::filesystem::path& folder, const std::tm& begin,
             const std::tm& end);

 private:
  std::string _host;
  uint16_t _port;
  Account _user;
  std::string _password;
};
}  // namespace email
}  // namespace palm
