#pragma once

#include <format>

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace aloe {
class PostgreSql {
  PostgreSql(const toml::table& node)
      : _host(node["host"].value_or("127.0.0.1")),
        _port(node["port"].value_or(5432)),
        _user(node["user"].value_or("postgres")),
        _password(node["password"].value<std::string>()),
        _db_name(node["name"].value<std::string>().value()) {}
  void dump();
  void restore(const std::string& name);
  inline std::string url() {
    return std::format("postgresql://{}:{}@{}:{}/{}", this->_user,
                       this->_password.value_or(""), this->_host, this->_port,
                       this->_db_name);
  }

  inline static const std::string SCHEMA_SQL = "schema.sql";
  inline static const std::string DATA_DUMP = "data.dump";

 private:
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::optional<std::string> _password;
  std::string _db_name;
};
}  // namespace aloe
