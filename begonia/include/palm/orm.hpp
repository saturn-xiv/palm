#pragma once

#include <chrono>
#include <cstdlib>
#include <optional>
#include <string>
#include <utility>

#include <boost/predef.h>

#include <libpq-fe.h>
#include <spdlog/spdlog.h>
#include <sqlite3.h>

// #if BOOST_ARCH_X86_64
// #include <mysql/mysql.h>
// #endif

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace palm {

// https://www.postgresql.org/docs/current/libpq.html
class PostgreSql {
 public:
  PostgreSql(const std::string& host, uint16_t port, const std::string& user,
             const std::optional<std::string> password,
             const std::string& db_name)
      : _host(host),
        _port(port),
        _user(user),
        _password(password),
        _db_name(db_name) {}

  //   https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
  std::string uri();

 private:
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::optional<std::string> _password;
  std::string _db_name;
};

// https://dev.mysql.com/downloads/c-api/
class MySql {
 public:
  MySql(const std::string& host, uint16_t port, const std::string& user,
        const std::optional<std::string> password, const std::string& db_name,
        const std::string& migrations_table)
      : _host(host),
        _port(port),
        _user(user),
        _password(password),
        _db_name(db_name) {}

 private:
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::optional<std::string> _password;
  std::string _db_name;
};

// https://www.sqlite.org/cintro.html
class Sqlite3 {
 public:
  Sqlite3(const std::string& file) : _file(file) {}

 private:
  std::string _file;
};
}  // namespace palm
