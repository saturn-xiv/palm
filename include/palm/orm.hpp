#pragma once

#include "palm/env.hpp"

#include <SQLiteCpp/SQLiteCpp.h>
#include <pqxx/pqxx>

namespace palm {

namespace orm {

class Migration {
 public:
  Migration() {}
  Migration(const std::filesystem::path& root);
  static void generate(const std::filesystem::path& root,
                       const std::string& name);

  static void header(std::ostream& out) {
    std::ios_base::fmtflags f(out.flags());
    out << std::left << std::setw(Migration::VERSION_SIZE) << "Version"
        << std::setw(Migration::NAME_SIZE) << "Name"
        << std::setw(Migration::RUN_AT_SIZE) << "Run At";
    out.flags(f);
  }

  friend std::ostream& operator<<(std::ostream& out, Migration const& self) {
    std::ios_base::fmtflags f(out.flags());
    out << std::left << std::setw(Migration::VERSION_SIZE) << self.version
        << std::setw(Migration::NAME_SIZE) << self.name
        << std::setw(Migration::RUN_AT_SIZE);
    if (self.run_at) {
      out << std::asctime(&self.run_at.value());
    } else {
      out << "n/a";
    }
    out.flags(f);
    return out;
  }

  friend struct sort_migration_asc;
  friend class Schema;

  inline static const std::string DIV = "-";
  inline static const std::string UP = "up.sql";
  inline static const std::string DOWN = "down.sql";
  inline static const std::string MIGRATION_FOLDER = "migrations";

  static const int VERSION_SIZE = 15;
  static const int NAME_SIZE = 36;
  static const int RUN_AT_SIZE = 24;

 private:
  int32_t id;
  std::string name;
  std::string version;
  std::string up;
  std::string down;
  std::optional<std::tm> run_at;
  std::tm created_at;
};

struct sort_migration_asc {
  inline bool operator()(const Migration& a, const Migration& b) {
    return (a.version < b.version);
  }
};

class Schema {
 public:
  Schema() {}
  static std::vector<Migration> load_migrations(
      const std::filesystem::path& root);
  static std::map<std::string, std::string> load_queries(
      const std::filesystem::path& root);

 private:
};

}  // namespace orm

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
