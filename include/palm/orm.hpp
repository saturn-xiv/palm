#pragma once

#include <chrono>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <optional>
#include <ostream>
#include <string>
#include <utility>
#include <vector>

#include <boost/property_tree/ptree.hpp>

#include <Poco/Data/Session.h>
#include <Poco/Data/SessionPool.h>

namespace palm {
namespace orm {
boost::property_tree::ptree queries(const std::filesystem::path& root);

class Migration {
 public:
  Migration(const std::filesystem::path& root);
  friend std::ostream& operator<<(std::ostream& out, const Migration& it) {
    out << it.version << "\t" << it.name;
    return out;
  }
  friend class Schema;

 private:
  // LANG=C date +%Y%m%d%H%M%S
  int64_t version;
  std::string name;
  std::string up;
  std::string down;
  Poco::Nullable<Poco::DateTime> run_at;
  Poco::DateTime created_at;
};

class Schema {
 public:
  Schema(Poco::Data::Session& db, const std::filesystem::path& root)
      : db(db), root(root) {}
  void init();
  void migrate();
  void rollback();
  friend std::ostream& operator<<(std::ostream& out, const Schema& it) {
    out << "\t";
    // TODO
    return out;
  }

 protected:
  virtual std::string latest() const = 0;
  virtual std::string down() const = 0;
  virtual std::string up() const = 0;
  virtual std::string select() const = 0;
  virtual std::string insert() const = 0;

 private:
  Poco::Data::Session db;
  std::filesystem::path root;
};
}  // namespace orm

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
Poco::Data::Session open(
    const std::filesystem::path& file,
    const std::chrono::seconds& timeout = std::chrono::seconds(5),
    const bool wal_mode = true);
}  // namespace sqlite3

/**
use DB-NAME
show tables;
desc TABLE-NAME;
SELECT table_name FROM information_schema.tables WHERE table_schema =
'databasename' AND table_name = 'testtable'; SHOW TABLES LIKE 'tablename';
*/
class MySQL {
 public:
  MySQL(const boost::property_tree::ptree& config);
  std::shared_ptr<Poco::Data::SessionPool> open() const;

 private:
  std::string host;
  uint16_t port;
  std::string database;
  std::string user;
  std::optional<std::string> password;
  std::chrono::seconds timeout;
  size_t pool_size;
};

/**
https://www.postgresql.org/docs/current/runtime-config-logging.html
/var/lib/postgres/data/postgresql.conf: log_statement = 'all'
sudo journalctl -u postgresql -f
*/
class PostgreSQL {
 public:
  PostgreSQL(const boost::property_tree::ptree& config);
  std::shared_ptr<Poco::Data::SessionPool> open() const;

 private:
  std::string host;
  uint16_t port;
  std::string database;
  std::string user;
  std::optional<std::string> password;
  std::chrono::seconds timeout;
  size_t pool_size;
};

}  // namespace palm
