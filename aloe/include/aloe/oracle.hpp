#pragma once

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace aloe {
class Oracle {
 public:
  Oracle(const toml::table& node)
      : _sid(node["sid"].value_or("orclcdb")),
        _user(node["user"].value_or("postgres")),
        _password(node["password"].value<std::string>().value()),
        _directory_path(node["directory-path"].value<std::string>().value()) {}
  void dump();
  void restore(const std::string& name, const std::string& user);

 private:
  std::string _sid;
  std::string _user;
  std::string _password;
  /**
   * select directory_path from dba_directories where
   * directory_name='DATA_PUMP_DIR'
   */
  std::string _directory_path;
};
}  // namespace aloe
