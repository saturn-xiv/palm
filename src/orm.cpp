#include "palm/orm.hpp"

#include <boost/algorithm/string.hpp>
#include <boost/property_tree/ini_parser.hpp>

palm::orm::Query::Query(const std::filesystem::path& root) {
  const auto file = root / "queries.ini";
  BOOST_LOG_TRIVIAL(debug) << "load queries from " << file.string();
  boost::property_tree::ini_parser::read_ini(file, this->tree);
  // for (const auto& section : tree) {
  //   for (const auto& node : section.second) {
  //     const std::string key = section.first + "." + node.first;
  //     BOOST_LOG_TRIVIAL(debug) << "find query: " << key;
  //     const std::optional<std::string> val =
  //         node.second.get_value<std::string>();
  //     instance[key] = val.value();
  //   }
  // }
}

palm::orm::migration::Migration::Migration(
    std::shared_ptr<soci::session> sql,
    const std::shared_ptr<palm::orm::Query> query,
    const std::filesystem::path& root)
    : sql(sql), query(query) {
  {
    std::ifstream it(root / "schema_migrations.sql");
    auto script = std::string(std::istreambuf_iterator<char>(it),
                              std::istreambuf_iterator<char>());
    boost::trim(script);
    *sql << script;
  }
  const auto node = root / MIGRATION_FOLDER;
  BOOST_LOG_TRIVIAL(debug) << "load db migrations from " << node.string();

  for (const auto& it : std::filesystem::directory_iterator(node)) {
    if (std::filesystem::is_directory(it)) {
      auto mig = Item(it);
      BOOST_LOG_TRIVIAL(debug)
          << "find migration " << mig.version << " " << mig.name;

      Item it;
      *sql << this->query->get("schema_migrations.by-version-and-name"),
          soci::use(mig), soci::into(it);

      if (sql->got_data()) {
        if (it.up != mig.up || it.down != mig.down) {
          std::stringstream ss;
          ss << "Migration [" << mig.version << "," << mig.name
             << "] already exists, however they aren't match";
          throw std::invalid_argument(ss.str());
        }
      } else {
        *sql << this->query->get("schema_migrations.insert"), soci::use(mig);
      }
    }
  }

  // std::sort(this->migrations.begin(), this->migrations.end(),
  //           palm::orm::migration::sort_by_asc());
}

void palm::orm::migration::Migration::migrate() {
  soci::transaction tr(*(this->sql));
  soci::rowset<Item> items =
      (this->sql->prepare << this->query->get("schema_migrations.all-asc"));

  for (const auto& it : items) {
    BOOST_LOG_TRIVIAL(info)
        << "found migration " << it.version << " " << it.name;
    if (it.run_at) {
      BOOST_LOG_TRIVIAL(info) << "ignore...";
      continue;
    }
    *(this->sql) << it.up;
    *(this->sql) << this->query->get("schema_migrations.set-run-at"),
        soci::use(it);
  }
  tr.commit();
}
void palm::orm::migration::Migration::rollback() {
  soci::transaction tr(*(this->sql));
  Item it;
  *(this->sql) << this->query->get("schema_migrations.latest"), soci::into(it);
  if (!sql->got_data()) {
    BOOST_LOG_TRIVIAL(debug) << "empty database";
    return;
  }

  BOOST_LOG_TRIVIAL(info) << "rollback migration " << it.version << " "
                          << it.name;

  *(this->sql) << it.down;
  *(this->sql) << this->query->get("schema_migrations.delete"), soci::use(it);
  tr.commit();
}

void palm::orm::migration::Migration::status(std::ostream& out) {
  {
    std::ios_base::fmtflags f(out.flags());
    out << std::left << std::setw(VERSION_SIZE) << "Version"
        << std::setw(NAME_SIZE) << "Name" << std::setw(RUN_AT_SIZE) << "Run At";
    out.flags(f);
  }
  out << std::endl;

  {
    soci::rowset<Item> items =
        (this->sql->prepare << this->query->get("schema_migrations.all-asc"));

    for (const auto& it : items) {
      out << it << std::endl;
    }
  }
}
void palm::orm::migration::Item::generate(const std::filesystem::path& root,
                                          const std::string& name) {
  std::stringstream ss;
  auto now = std::time(nullptr);
  ss << std::put_time(std::gmtime(&now), "%Y%m%d%H%M%S") << DIV << name;
  const auto node = root / MIGRATION_FOLDER / ss.str();
  BOOST_LOG_TRIVIAL(info) << "create migration in folder " << node;
  std::filesystem::create_directories(node);
  {
    std::ofstream it(node / UP);
    it.close();
  }
  {
    std::ofstream it(node / DOWN);
    it.close();
  }
}

palm::orm::migration::Item::Item(const std::filesystem::path& root) {
  BOOST_LOG_TRIVIAL(info) << "load migration from " << root;
  {
    std::ifstream it(root / UP);
    this->up = std::string(std::istreambuf_iterator<char>(it),
                           std::istreambuf_iterator<char>());
    boost::trim(this->up);
  }
  {
    std::ifstream it(root / DOWN);
    this->down = std::string(std::istreambuf_iterator<char>(it),
                             std::istreambuf_iterator<char>());
    boost::trim(this->down);
  }

  const auto fn = root.filename().string();
  const auto pos = fn.find(DIV);
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

std::string palm::postgresql::Config::url() const {
  std::stringstream ss;
  ss << "host=" << this->host;
  ss << " port=" << this->port;
  ss << " dbname=" << this->name;
  ss << " user=" << this->user;
  if (password) {
    ss << " password=" << password.value();
  }
  ss << " requiressl=0";
  return ss.str();
}

std::shared_ptr<pqxx::connection> palm::postgresql::Config::open() const {
  BOOST_LOG_TRIVIAL(debug) << "connect to " << this->user << "@" << this->host
                           << ":" << this->port << "/" << this->name;

  const auto url = this->url();
  std::shared_ptr<pqxx::connection> con =
      std::make_shared<pqxx::connection>(url);
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

std::shared_ptr<soci::connection_pool> palm::orm::pool::open(
    const soci::backend_factory& backend, const std::string& url,
    const size_t size) {
  std::shared_ptr<soci::connection_pool> it =
      std::make_shared<soci::connection_pool>(size);
  for (auto i = 0; i < size; i++) {
    soci::session& sql = it->at(i);
    sql.open(backend, url);
    sql.set_logger(new palm::orm::logger());
  }
  return it;
}
