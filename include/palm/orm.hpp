#pragma once

#include "palm/env.hpp"

#include <boost/property_tree/ptree.hpp>

#include <SQLiteCpp/SQLiteCpp.h>
#include <pqxx/pqxx>


#include <soci/postgresql/soci-postgresql.h>
#include <soci/sqlite3/soci-sqlite3.h>

namespace palm {

namespace orm {

class logger : public soci::logger_impl {
 public:
  void start_query(std::string const& query) {
    BOOST_LOG_TRIVIAL(debug) << query;
  }

 private:
  logger_impl* do_clone() const { return new logger(); }
};

class Query {
 public:
  Query(const std::filesystem::path& root);
  inline std::string get(const std::string& name) {
    return this->tree.get<std::string>(name);
  }

 private:
  boost::property_tree::ptree tree;
};

namespace migration {

inline static const std::string DIV = "-";
inline static const std::string UP = "up.sql";
inline static const std::string DOWN = "down.sql";
inline static const std::string MIGRATION_FOLDER = "migrations";

static const int VERSION_SIZE = 15;
static const int NAME_SIZE = 36;
static const int RUN_AT_SIZE = 24;

class Item {
 public:
  Item() {}
  Item(const std::filesystem::path& root);
  static void generate(const std::filesystem::path& root,
                       const std::string& name);

  friend std::ostream& operator<<(std::ostream& out, const Item& self) {
    std::ios_base::fmtflags f(out.flags());
    out << std::left << std::setw(VERSION_SIZE) << self.version
        << std::setw(NAME_SIZE) << self.name << std::setw(RUN_AT_SIZE);
    if (self.run_at) {
      // out << std::asctime(&self.run_at.value());
      out << std::put_time(&self.run_at.value(), "%c %Z");
    } else {
      out << "n/a";
    }
    out.flags(f);
    return out;
  }

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Item, id, name, version, up, down, run_at,
                                 created_at)

 public:
  int32_t id;
  std::string name;
  std::string version;
  std::string up;
  std::string down;
  boost::optional<std::tm> run_at;
  std::tm created_at;
};

struct sort_by_asc {
  inline bool operator()(const Item& a, const Item& b) {
    return (a.version < b.version);
  }
};

class Migration {
 public:
  Migration(std::shared_ptr<soci::session> sql,
            const std::shared_ptr<palm::orm::Query> query,
            const std::filesystem::path& root);
  void migrate();
  void rollback();
  void status(std::ostream& out);

 private:
  std::shared_ptr<soci::session> sql;
  std::shared_ptr<palm::orm::Query> query;
};

}  // namespace migration

namespace pool {
std::shared_ptr<soci::connection_pool> open(
    const soci::backend_factory& backend, const std::string& url,
    const size_t size);
}

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
}  // namespace sqlite

// https://www.postgresql.org/docs/current/runtime-config-logging.html
// /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
namespace postgresql {

class Config {
 public:
  Config(const std::string& name = "demo",
         const std::string& host = "127.0.0.1", const uint16_t port = 5432,
         const std::string& user = "postgres",
         const std::optional<std::string>& password = std::nullopt)
      : host(host), port(port), name(name), user(user), password(password) {}
  std::shared_ptr<pqxx::connection> open() const;
  std::string url() const;

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
  Config(const std::string& name = "demo",
         const std::string& host = "127.0.0.1", const uint16_t port = 3306,
         const std::string& user = "root",
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

namespace soci {
template <>
struct type_conversion<palm::orm::migration::Item> {
  typedef values base_type;

  static void from_base(values const& v, indicator ind,
                        palm::orm::migration::Item& p) {
    p.id = v.get<int32_t>("id");
    p.name = v.get<std::string>("name");
    p.version = v.get<std::string>("version");
    p.up = v.get<std::string>("up");
    p.down = v.get<std::string>("down");

    p.run_at = v.get<boost::optional<std::tm>>("run_at");
    // if (v.get_indicator("run_at") == i_null) {
    //   p.run_at = std::nullopt;
    // } else {
    //   p.run_at = v.get<std::tm>("run_at");
    // }
    p.created_at = v.get<std::tm>("created_at");
  }

  static void to_base(const palm::orm::migration::Item& p, values& v,
                      indicator& ind) {
    v.set("id", p.id);
    v.set("name", p.name);
    v.set("version", p.version);
    v.set("up", p.up);
    v.set("down", p.down);

    v.set("run_at", p.run_at);
    // if (p.run_at) {
    //   v.set("run_at", p.run_at.value());
    // } else {
    //   v.set("run_at", NULL, i_null);
    // }
    v.set("created_at", p.created_at);
    ind = i_ok;
  }
};
}  // namespace soci
