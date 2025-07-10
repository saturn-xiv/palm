#pragma once

#include <chrono>
#include <cstdlib>
#include <optional>
#include <string>
#include <utility>

#include <boost/predef.h>

#define SOCI_USE_BOOST
#include <soci/soci.h>

#include <soci/postgresql/soci-postgresql.h>
#include <soci/sqlite3/soci-sqlite3.h>
#include <spdlog/spdlog.h>

// #if BOOST_ARCH_X86_64
// #include <mysql/mysql.h>
// #endif

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace palm {

std::pair<int, int> paginate(int total, int index = 1, int size = 60);

class SociLogger : public soci::logger_impl {
 public:
  void start_query(std::string const& query) { spdlog::debug("{}", query); }

 private:
  SociLogger* do_clone() const { return new SociLogger(); }
};

// https://www.postgresql.org/docs/current/libpq.html
class PostgreSql {
 public:
  PostgreSql(const std::string& host, uint16_t port, const std::string& user,
             const std::optional<std::string> password,
             const std::string& db_name, size_t pool_size = (1 << 5))
      : _host(host),
        _port(port),
        _user(user),
        _password(password),
        _db_name(db_name),
        _pool_size(pool_size) {}
  PostgreSql(const toml::table& config)
      : _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(5432)),
        _user(config["user"].value_or<std::string>("postgres")),
        _password(config["password"].value<std::string>()),
        _db_name(config["db-name"].value<std::string>().value()),
        _pool_size(config["pool-size"].value_or<size_t>(1 << 5)) {}
  //   https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
  std::string uri() const;
  std::shared_ptr<soci::connection_pool> open() const;

 private:
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::optional<std::string> _password;
  std::string _db_name;
  size_t _pool_size;
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
