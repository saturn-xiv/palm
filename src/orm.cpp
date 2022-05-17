#include "palm/orm.hpp"

#include <boost/date_time/gregorian/formatters.hpp>
#include <boost/foreach.hpp>
#include <boost/log/trivial.hpp>
#include <boost/property_tree/ini_parser.hpp>

#include <Poco/Data/MySQL/Connector.h>
#include <Poco/Data/PostgreSQL/Connector.h>
#include <Poco/Data/SQLite/Connector.h>
#include <Poco/Data/Transaction.h>

std::shared_ptr<Poco::Data::SessionPool> palm::PostgreSQL::open() const {
  Poco::Data::PostgreSQL::Connector::registerConnector();
  BOOST_LOG_TRIVIAL(info) << "open postgresql " << this->user << '@'
                          << this->host << ':' << this->port << '/'
                          << this->database;
  // https://www.postgresql.org/docs/14/libpq-connect.html#LIBPQ-CONNSTRING
  // https://docs.pocoproject.org/current/Poco.Data.PostgreSQL.SessionImpl.html
  std::stringstream url;
  {
    url << "host=" << this->host;
    url << " port=" << this->port;
    url << " dbname=" << this->database;
    url << " user=" << this->user;
    if (this->password) {
      url << " password=" << this->password.value();
    }
    url << " sslmode=disable";
    url << " connect_timeout=" << this->timeout.count();
  }

  std::shared_ptr<Poco::Data::SessionPool> pool =
      std::make_shared<Poco::Data::SessionPool>("PostgreSQL", url.str());
  return pool;
}

palm::PostgreSQL::PostgreSQL(const boost::property_tree::ptree& config) {
  this->host = config.get("postgresql.host", "127.0.0.1");
  this->port = config.get("postgresql.port", 5432);
  this->user = config.get("postgresql.user", "postgres");
  {
    auto it = config.get_optional<std::string>("postgresql.password");
    if (it.has_value()) {
      this->password = it.get();
    }
  }
  this->database = config.get<std::string>("postgresql.db-name");
  {
    size_t it = config.get("postgresql.connection-timeout", 12);
    this->timeout = std::chrono::seconds(it);
  }
  this->pool_size = config.get("postgresql.pool-size", 32);
}

palm::MySQL::MySQL(const boost::property_tree::ptree& config) {
  this->host = config.get("mysql.host", "127.0.0.1");
  this->port = config.get("mysql.port", 3306);
  this->user = config.get("mysql.user", "root");
  {
    auto it = config.get_optional<std::string>("mysql.password");
    if (it.has_value()) {
      this->password = it.get();
    }
  }
  this->database = config.get<std::string>("mysql.db-name");
  {
    size_t it = config.get("mysql.connection-timeout", 12);
    this->timeout = std::chrono::seconds(it);
  }
  this->pool_size = config.get("mysql.pool-size", 32);
}

// https://docs.pocoproject.org/current/Poco.Data.MySQL.SessionImpl.html
std::shared_ptr<Poco::Data::SessionPool> palm::MySQL::open() const {
  Poco::Data::MySQL::Connector::registerConnector();
  BOOST_LOG_TRIVIAL(info) << "open mysql " << this->user << '@' << this->host
                          << ':' << this->port << '/' << this->database;
  std::stringstream url;
  {
    url << "host=" << this->host;
    url << "; port=" << this->port;
    url << "; db=" << this->database;
    url << "; user=" << this->user;
    if (this->password) {
      url << "; password=" << this->password.value();
    }
    url << "; character-set=utf8";
    url << "; auto-reconnect=true";
  }

  std::shared_ptr<Poco::Data::SessionPool> pool =
      std::make_shared<Poco::Data::SessionPool>("MySQL", url.str());
  return pool;
}

Poco::Data::Session palm::sqlite3::open(const std::filesystem::path& file,
                                        const std::chrono::seconds& timeout,
                                        const bool wal_mode) {
  Poco::Data::SQLite::Connector::registerConnector();
  std::stringstream url;
  {
    url << file.string();
    // url << "db=" << file.string();
    // url << " timeout=" << timeout.count();
    // url << " shared_cache=true";
    // url << " synchronous=off";
  }
  BOOST_LOG_TRIVIAL(info) << "open sqlite3 " << file;
  Poco::Data::Session db("SQLite", url.str());
  return db;
}

void palm::orm::Schema::migrate() {
  Poco::Data::Transaction db(this->db);
  // TODO
  db.commit();
}

void palm::orm::Schema::rollback() {
  // TODO
}

boost::property_tree::ptree palm::orm::queries(
    const std::filesystem::path& root) {
  boost::property_tree::ptree tree;
  for (const auto& it : std::filesystem::directory_iterator(root / "queries")) {
    const std::string file = it.path();
    BOOST_LOG_TRIVIAL(debug) << "load query file from " << file;
    boost::property_tree::ptree pt;
    boost::property_tree::read_ini(file, pt);
    BOOST_FOREACH (auto& t, pt) { tree.put_child(t.first, t.second); }
  }
  return tree;
}

void palm::orm::Schema::init() {
  const auto top = this->root / "migrations";

  {
    std::ifstream fs;
    std::ios_base::iostate mask = fs.exceptions() | std::ios::failbit;
    fs.exceptions(mask);
    fs.open(top / "initial-setup.sql");
    std::string sql((std::istreambuf_iterator<char>(fs)),
                    std::istreambuf_iterator<char>());
    this->db << sql;
  }

  const auto sql_s = this->select();
  const auto sql_i = this->insert();
  for (const auto& it : std::filesystem::directory_iterator(top)) {
    const auto node = it.path();
    if (!std::filesystem::is_directory(node)) {
      continue;
    }
    palm::orm::Migration mig(node);

    {
      Poco::Nullable<std::string> name;
      Poco::Nullable<std::string> up;
      Poco::Nullable<std::string> down;
      {
        Poco::Data::Statement st(db);
        st << sql_s, Poco::Data::Keywords::use(mig.version),
            Poco::Data::Keywords::into(up), Poco::Data::Keywords::into(down),
            Poco::Data::Keywords::now;
        st.execute();
      }
      if (up.isNull() && down.isNull()) {
        Poco::Data::Statement st(db);
        st << sql_i, Poco::Data::Keywords::use(mig.version),
            Poco::Data::Keywords::use(mig.name),
            Poco::Data::Keywords::use(mig.up),
            Poco::Data::Keywords::use(mig.down), Poco::Data::Keywords::now;
        st.execute();
        continue;
      }
      if (name.value() != mig.name || up.value() != mig.up ||
          down.value() != mig.down) {
        throw std::invalid_argument("migration not match in db");
      }
    }
  }
}

palm::orm::Migration::Migration(const std::filesystem::path& root) {
  {
    std::ifstream fs;
    std::ios_base::iostate mask = fs.exceptions() | std::ios::failbit;
    fs.exceptions(mask);
    fs.open(root / "up.sql");
    this->up = std::string((std::istreambuf_iterator<char>(fs)),
                           std::istreambuf_iterator<char>());
  }
  {
    std::ifstream fs;
    std::ios_base::iostate mask = fs.exceptions() | std::ios::failbit;
    fs.exceptions(mask);
    fs.open(root / "down.sql");
    this->down = std::string((std::istreambuf_iterator<char>(fs)),
                             std::istreambuf_iterator<char>());
  }
}
