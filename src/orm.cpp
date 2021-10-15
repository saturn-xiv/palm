#include "palm/orm.hpp"

#include <boost/algorithm/string.hpp>
#include <boost/property_tree/ini_parser.hpp>
#include <boost/property_tree/ptree.hpp>

static void loop_ini_tree(std::map<std::string, std::string>& items,
                          const std::optional<std::string>& key,
                          const boost::property_tree::ptree& val) {
  {
    const std::optional<std::string> v = val.get_value<std::string>();
    if (v) {
      items[key.value()] = v.value();
      return;
    }
  }
  for (const auto& it : val) {
    const std::string k = key ? (key.value() + "." + it.first) : it.first;
    loop_ini_tree(items, k, it.second);
  }
}

std::map<std::string, std::string> palm::orm::Schema::load_queries(
    const std::filesystem::path& root) {
  auto file = root / "queries.ini";
  BOOST_LOG_TRIVIAL(debug) << "load queries from " << file.string();
  boost::property_tree::ptree queries;
  boost::property_tree::ini_parser::read_ini(file, queries);

  std::map<std::string, std::string> items;
  loop_ini_tree(items, std::nullopt, queries);
  return items;
}

std::vector<palm::orm::Migration> palm::orm::Schema::load_migrations(
    const std::filesystem::path& root) {
  std::vector<palm::orm::Migration> items;

  const auto node = root / Migration::MIGRATION_FOLDER;

  BOOST_LOG_TRIVIAL(debug) << "load db migrations from " << node.string();
  for (const auto& it : std::filesystem::directory_iterator(node)) {
    if (std::filesystem::is_directory(it)) {
      auto mig = Migration(it);
      BOOST_LOG_TRIVIAL(debug)
          << "find migration " << mig.version << " " << mig.name;
      items.push_back(mig);
    }
  }
  std::sort(items.begin(), items.end(), palm::orm::sort_migration_asc());
  return items;
}

void palm::orm::Migration::generate(const std::filesystem::path& root,
                                    const std::string& name) {
  std::stringstream ss;
  auto now = std::time(nullptr);
  ss << std::put_time(std::gmtime(&now), "%Y%m%d%H%M%S") << Migration::DIV
     << name;
  const auto node = root / Migration::MIGRATION_FOLDER / ss.str();
  BOOST_LOG_TRIVIAL(info) << "create migration in folder " << node;
  std::filesystem::create_directories(node);
  {
    std::ofstream it(node / Migration::UP);
    it.close();
  }
  {
    std::ofstream it(node / Migration::DOWN);
    it.close();
  }
}

palm::orm::Migration::Migration(const std::filesystem::path& root) {
  BOOST_LOG_TRIVIAL(info) << "load migration from " << root;
  {
    std::ifstream it(root / Migration::UP);
    this->up = std::string((std::istreambuf_iterator<char>(it)),
                           std::istreambuf_iterator<char>());
    boost::trim(this->up);
  }
  {
    std::ifstream it(root / Migration::DOWN);
    this->down = std::string((std::istreambuf_iterator<char>(it)),
                             std::istreambuf_iterator<char>());
    boost::trim(this->down);
  }

  const auto fn = root.filename().string();
  const auto pos = fn.find(Migration::DIV);
  if (pos == std::string::npos) {
    throw std::invalid_argument("bad folder name");
  }
  this->version = fn.substr(0, pos);
  this->name = fn.substr(pos + 1);
}

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
