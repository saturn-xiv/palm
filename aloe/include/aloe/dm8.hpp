#pragma once

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace aloe {
class Dm8 {
  Dm8(const toml::table& node)
      : _port(node["port"].value_or(5236)),
        _user(node["user"].value_or("sysdba")),
        _password(node["password"].value<std::string>().value()) {}
  void dump(const std::string& directory);
  void restore(const std::string& directory, const std::string& name);

 private:
  uint16_t _port;
  std::string _user;
  std::string _password;
};
}  // namespace aloe
