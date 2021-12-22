#pragma once

#include "palm/env.hpp"

#include <chrono>
#include <cstdint>
#include <ctime>
#include <mutex>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <unordered_map>
#include <vector>

#include <boost/log/trivial.hpp>
#include <boost/property_tree/ptree.hpp>

#define SOCI_USE_BOOST
#include <soci/soci.h>

namespace palm {

namespace sqlite3 {

/**
 .show Displays current settings for various parameters
 .databases Provides database names and files
 .quit Quit sqlite3 program
 .tables Show current tables
 .schema Display schema of table
 .header Display or hide the output table header
 .mode Select mode for the output table
 .dump Dump database in SQL text format
 pragma compile_options;
 SELECT name FROM sqlite_master WHERE type='table' AND name='TABLE_NAME'
*/
std::shared_ptr<soci::session> open(
    const std::filesystem::path& file,
    const std::chrono::seconds& timeout = std::chrono::seconds(5),
    const bool wal_mode = true);
}  // namespace sqlite3

namespace mysql {
/**
use DB-NAME
show tables;
desc TABLE-NAME;
SELECT table_name FROM information_schema.tables WHERE table_schema =
'databasename' AND table_name = 'testtable'; SHOW TABLES LIKE 'tablename';
*/
}

namespace postgresql {
/**
https://www.postgresql.org/docs/current/runtime-config-logging.html
/var/lib/postgres/data/postgresql.conf: log_statement = 'all'
sudo journalctl -u postgresql -f
*/
std::shared_ptr<soci::session> open(
    const std::string& name, const std::string& host = "127.0.0.1",
    const uint16_t port = 5432, const std::string& user = "postgres",
    const std::optional<std::string> password = std::nullopt);
}  // namespace postgresql

namespace orm {
class Logger : public soci::logger_impl {
 public:
  virtual void start_query(std::string const& query) {
    BOOST_LOG_TRIVIAL(info) << query;
  }

 private:
  virtual soci::logger_impl* do_clone() const { return new Logger(); }
};

class Query {
 public:
  Query(Query const&) = delete;
  void operator=(Query const&) = delete;

  static Query& instance() {
    static Query it;
    return it;
  }
  inline std::string get(const std::string& key) const {
    for (const auto& [_, tree] : this->trees) {
      const auto it = tree.get_optional<std::string>(key);
      if (it) {
        return it.value();
      }
    }
    {
      std::stringstream ss;
      ss << "can't find query " << key;
      throw std::runtime_error(ss.str());
    }
  }

  void load(const std::filesystem::path& file);

 private:
  Query() {}
  std::mutex locker;
  std::unordered_map<std::string, boost::property_tree::ptree> trees;
};

struct Migration {
  friend std::ostream& operator<<(std::ostream& out, const Migration& it) {
    out << it.version << "\t" << it.name;
    return out;
  }

  std::string up;
  std::string down;
  // LANG=C date +%Y%m%d%H%M%S
  std::string version;
  std::string name;
  boost::optional<std::tm> run_on;
  std::tm created_at;
};
class Schema {
 public:
  Schema(const std::filesystem::path& root, std::shared_ptr<soci::session> db);
  void migrate();
  void rollback();
  void status(std::ostream& out);

 private:
  std::vector<Migration> migrations;
  std::shared_ptr<soci::session> db;
};
}  // namespace orm
}  // namespace palm

namespace soci {
template <>
struct type_conversion<palm::orm::Migration> {
  typedef soci::values base_type;

  static void from_base(const soci::values& v, soci::indicator i,
                        palm::orm::Migration& o) {
    o.version = v.get<std::string>("version");
    o.name = v.get<std::string>("name");
    o.up = v.get<std::string>("up");
    o.down = v.get<std::string>("down");
    o.run_on = v.get<boost::optional<std::tm>>("run_on");
    o.created_at = v.get<std::tm>("created_at");
  }

  static void to_base(const palm::orm::Migration& o, soci::values& v,
                      soci::indicator& i) {
    v.set("version", o.version);
    v.set("name", o.name);
    v.set("up", o.up);
    v.set("down", o.down);
    v.set("run_on", o.run_on,
          o.run_on.is_initialized() ? soci::i_ok : soci::i_null);
    v.set("created_at", o.created_at);

    i = soci::i_ok;
  }
};
}  // namespace soci
