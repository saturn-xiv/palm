#pragma once

#include <chrono>
#include <cstdlib>
#include <optional>
#include <string>
#include <utility>

#include <boost/predef.h>

#include <soci/boost-fusion.h>
#include <soci/boost-gregorian-date.h>
#include <soci/boost-optional.h>
#include <soci/boost-tuple.h>
#include <soci/connection-pool.h>
#include <soci/postgresql/soci-postgresql.h>
#include <soci/session.h>
#include <soci/sqlite3/soci-sqlite3.h>
#include <spdlog/spdlog.h>

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
  PostgreSql(toml::table* config)
      : _host(config->get("host")->value_or<std::string>("127.0.0.1")),
        _port(config->get("port")->value_or<uint16_t>(5672)),
        _user(config->get("user")->value_or<std::string>("postgres")),
        _password(config->get("password")->value<std::string>()),
        _db_name(config->get("db-name")->value<std::string>().value()) {}
  //   https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
  std::string uri() const;
  std::shared_ptr<soci::connection_pool> open(size_t pool_size = (1
                                                                  << 5)) const;

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
  Sqlite3(const std::string& file, size_t timeout = 5)
      : _file(file), _timeout(timeout) {}
  std::shared_ptr<soci::session> open() const;

 private:
  std::string _file;
  size_t _timeout;
};
}  // namespace palm
