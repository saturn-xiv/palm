#pragma once

#define SOCI_USE_BOOST
#include <soci/soci.h>

#include <soci/postgresql/soci-postgresql.h>
#include <spdlog/spdlog.h>
#include <toml++/toml.hpp>

namespace palm {
class SociLogger : public soci::logger_impl {
 public:
  void start_query(std::string const& query) { spdlog::debug("{}", query); }

 private:
  SociLogger* do_clone() const { return new SociLogger(); }
};

class PostgreSql {
 public:
  PostgreSql(const toml::table& config)
      : _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(5432)),
        _user(config["user"].value_or<std::string>("postgres")),
        _password(config["password"].value<std::string>()),
        _db_name(config["db-name"].value_or<std::string>("/")),
        _pool_size(config["pool-size"].value_or<size_t>(1 << 5)) {}
  PostgreSql(const std::string& host = "127.0.0.1", uint16_t port = 5432,
             const std::string& user = "postgres",
             const std::optional<std::string> password = std::nullopt,
             const std::string& db_name = "postgres",
             size_t pool_size = (1 << 5))
      : _host(host),
        _port(port),
        _user(user),
        _password(password),
        _db_name(db_name),
        _pool_size(pool_size) {}

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
}  // namespace palm
