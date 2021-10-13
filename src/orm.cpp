#include "palm/orm.hpp"

// https://stackoverflow.com/questions/57123453/how-to-use-diesel-with-sqlite-connections-and-avoid-database-is-locked-type-of
std::shared_ptr<SQLite::Database> palm::sqlite::open(
    const std::filesystem::path& file, const bool wal_mode,
    const std::optional<std::chrono::seconds>& busy_timeout) {
  BOOST_LOG_TRIVIAL(info) << "open sqlite3 database " << file;
  std::shared_ptr<SQLite::Database> db = std::make_shared<SQLite::Database>(
      file.string(), SQLite::OPEN_READWRITE | SQLite::OPEN_CREATE);
  {
    SQLite::Statement query(*db, "SELECT SQLITE_VERSION() AS value");
    if (query.executeStep()) {
      const std::string version = query.getColumn("value");
      BOOST_LOG_TRIVIAL(debug) << "sqlite version: " << version;
    }
  }
  db->exec("PRAGMA synchronous = OFF");
  if (wal_mode) {
    BOOST_LOG_TRIVIAL(info) << "enable wal journal mode";
    db->exec("PRAGMA journal_mode = WAL");
  }
  db->exec("PRAGMA foreign_keys = ON");
  if (busy_timeout) {
    const auto to = std::chrono::duration_cast<std::chrono::milliseconds>(
        busy_timeout.value());
    BOOST_LOG_TRIVIAL(info) << "set busy timeout to " << to.count();
    std::stringstream sql;
    sql << "PRAGMA busy_timeout = " << to.count();
    db->exec(sql.str());
    // SQLite::Statement query(*db, "PRAGMA busy_timeout = :value");
    // query.bind(":value", to.count());
    // query.exec();
  }
  return db;
}

std::shared_ptr<pqxx::connection> palm::postgresql::Config::open() const {
  BOOST_LOG_TRIVIAL(debug) << "connect to " << this->user << "@" << this->host
                           << ":" << this->port << "/" << this->name;
  std::stringstream url;
  {
    url << "host=" << this->host;
    url << " port=" << this->port;
    url << " dbname=" << this->name;
    url << " user=" << this->user;
    if (password) {
      url << " password=" << password.value();
    }
    url << " requiressl=0";
  }
  std::shared_ptr<pqxx::connection> con =
      std::make_shared<pqxx::connection>(url.str());
  {
    pqxx::work tr{*con};
    pqxx::result rst{tr.exec("SELECT VERSION() AS value")};
    const auto& row = rst.front();
    const std::string version = row["value"].as(std::string());
    BOOST_LOG_TRIVIAL(debug) << "version: " << version;
    tr.commit();
  }
  return con;
}
