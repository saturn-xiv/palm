#pragma once

#include <filesystem>
#include <format>

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace aloe {
class Dm8 {
 public:
  Dm8(const toml::table& node)
      : _host(node["host"].value_or("127.0.0.1")),
        _port(node["port"].value_or(5236)),
        _user(node["user"].value_or("sysdba")),
        _password(node["password"].value<std::string>().value()),
        _home(node["home"].value_or("/opt/dmdbms")) {}
  void dump(const std::string& directory,bool zip);
  void restore(const std::string& directory, const std::filesystem::path& file);

 private:
  inline std::string _url() {
    return std::format("{}/{}@{}:{}", this->_user, this->_password, this->_host,
                       this->_port);
  }
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::string _password;
  std::filesystem::path _home;
};
}  // namespace aloe
