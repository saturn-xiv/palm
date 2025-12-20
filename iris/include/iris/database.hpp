#pragma once

#include "iris/env.hpp"

namespace iris {
class PostgreSql : public Storage {
 public:
  PostgreSql(const toml::table& config)
      : _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(5432)),
        _user(config["user"].value_or<std::string>("postgres")),
        _password(config["password"].value<std::string>()),
        _db_name(config["db-name"].value<std::string>().value()) {}

  void dump(const std::filesystem::path& output) const override;
  void restore(const std::filesystem::path& file) const override;

 private:
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::optional<std::string> _password;
  std::string _db_name;
};

class MySql : public Storage {
 public:
  MySql(const toml::table& config)
      : _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(3306)),
        _user(config["user"].value_or<std::string>("root")),
        _password(config["password"].value<std::string>()),
        _db_name(config["db-name"].value<std::string>().value()) {}

  void dump(const std::filesystem::path& output) const override;
  void restore(const std::filesystem::path& file) const override;

 private:
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::optional<std::string> _password;
  std::string _db_name;
};

class Dm8 : public Storage {
 public:
  Dm8(const toml::table& config)
      : _home(config["home"].value_or<std::string>("/opt/dm8")),
        _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(5236)),
        _user(config["user"].value_or<std::string>("SYSDBA")),
        _password(config["password"].value<std::string>()) {}

  void dump(const std::filesystem::path& output) const override;
  void restore(const std::filesystem::path& file) const override;

 private:
  std::string _home;
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::optional<std::string> _password;
};

// https://oracle-base.com/articles/10g/oracle-data-pump-10g
// https://support.oracle.com/knowledge/Oracle%20Database%20Products/2620296_1.html
class Oracle : public Storage {
 public:
  Oracle(const toml::table& config)
      : _home(config["home"].value_or<std::string>("/opt/oracle")),
        _sid(config["sid"].value_or<std::string>("orcl")),
        _user(config["user"].value_or<std::string>("sysdba")),
        _password(config["password"].value<std::string>()) {}

  void dump(const std::filesystem::path& output) const override;
  void restore(const std::filesystem::path& file) const override;

 private:
  std::string _home;
  std::string _sid;
  std::string _user;
  std::optional<std::string> _password;
};
}  // namespace iris
