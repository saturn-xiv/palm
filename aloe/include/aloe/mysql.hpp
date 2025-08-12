#pragma once

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace aloe {
class MySql {
  public:
  MySql(const toml::table& node)
      : _host(node["host"].value_or("127.0.0.1")),
        _port(node["port"].value_or(3306)),
        _user(node["user"].value_or("root")),
        _password(node["password"].value<std::string>()),
        _db_name(node["name"].value<std::string>().value()) {}
  void dump();
  void restore(const std::string& name);

 private:
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::optional<std::string> _password;
  std::string _db_name;
};
}  // namespace aloe
