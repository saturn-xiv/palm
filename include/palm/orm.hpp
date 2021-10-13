#pragma once

#include "palm/env.hpp"

#include <SQLiteCpp/SQLiteCpp.h>
// #include <mariadb/conncpp.hpp>
#include <pqxx/pqxx>

namespace palm {
// .show Displays current settings for various parameters
// .databases Provides database names and files
// .quit Quit sqlite3 program
// .tables Show current tables
// .schema Display schema of table
// .header Display or hide the output table header
// .mode Select mode for the output table
// .dump Dump database in SQL text format
// pragma compile_options;
// SELECT name FROM sqlite_master WHERE type='table' AND name='TABLE_NAME'
//
namespace sqlite {
std::shared_ptr<SQLite::Database> open(
    const std::filesystem::path& db, const bool wal_mode = true,
    const std::optional<std::chrono::seconds>& busy_timeout =
        std::chrono::seconds(5));
}
// https://www.postgresql.org/docs/current/runtime-config-logging.html
// /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
namespace postgresql {
class Config {
 public:
  Config() {}
  Config(const std::string& name, const std::string& host = "127.0.0.1",
         const uint16_t port = 5432, const std::string& user = "postgres",
         const std::optional<std::string>& password = std::nullopt)
      : host(host), port(port), name(name), user(user), password(password) {}
  std::shared_ptr<pqxx::connection> open() const;

  void operator=(const toml::table& node) {
    {
      auto it = node["host"].value<std::string>();
      if (it) {
        this->host = it.value();
      }
    }
    {
      auto it = node["port"].value<uint16_t>();
      if (it) {
        this->port = it.value();
      }
    }
    {
      auto it = node["name"].value<std::string>();
      if (it) {
        this->name = it.value();
      }
    }
    {
      auto it = node["user"].value<std::string>();
      if (it) {
        this->user = it.value();
      }
    }
    {
      auto it = node["password"].value<std::string>();
      if (it) {
        this->password = it.value();
      }
    }
  }
  friend toml::table& operator<<(toml::table& node, const Config& self) {
    node.insert_or_assign("host", self.host);
    node.insert_or_assign("port", self.port);
    node.insert_or_assign("name", self.name);
    node.insert_or_assign("user", self.user);
    if (self.password) {
      node.insert_or_assign("password", self.password.value());
    }
    return node;
  }

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Config, host, port, user, password)
 private:
  std::string host;
  uint16_t port;
  std::string user;
  std::string name;
  std::optional<std::string> password;
};
}  // namespace postgresql
// use DB-NAME
// show tables;
// desc TABLE-NAME;
// SELECT table_name FROM information_schema.tables WHERE table_schema =
// 'databasename' AND table_name = 'testtable'; SHOW TABLES LIKE 'tablename';
// SELECT VERSION() AS value
namespace mysql {
class Config {
 public:
  Config() {}
  Config(const std::string& name, const std::string& host = "127.0.0.1",
         const uint16_t port = 3306, const std::string& user = "root",
         const std::optional<std::string>& password = std::nullopt)
      : host(host), port(port), name(name), user(user), password(password) {}
  //   https://mariadb.com/resources/blog/how-to-connect-c-programs-to-mariadb/

  void operator=(const toml::table& node) {
    {
      auto it = node["host"].value<std::string>();
      if (it) {
        this->host = it.value();
      }
    }
    {
      auto it = node["port"].value<uint16_t>();
      if (it) {
        this->port = it.value();
      }
    }
    {
      auto it = node["name"].value<std::string>();
      if (it) {
        this->name = it.value();
      }
    }
    {
      auto it = node["user"].value<std::string>();
      if (it) {
        this->user = it.value();
      }
    }
    {
      auto it = node["password"].value<std::string>();
      if (it) {
        this->password = it.value();
      }
    }
  }
  friend toml::table& operator<<(toml::table& node, const Config& self) {
    node.insert_or_assign("host", self.host);
    node.insert_or_assign("port", self.port);
    node.insert_or_assign("name", self.name);
    node.insert_or_assign("user", self.user);
    if (self.password) {
      node.insert_or_assign("password", self.password.value());
    }
    return node;
  }

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Config, host, port, user, password)
 private:
  std::string host;
  std::string name;
  uint16_t port;
  std::string user;
  std::optional<std::string> password;
};
}  // namespace mysql
}  // namespace palm
