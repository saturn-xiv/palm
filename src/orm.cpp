#include "palm/orm.hpp"

#include <soci/postgresql/soci-postgresql.h>
#include <soci/sqlite3/soci-sqlite3.h>
#include <boost/property_tree/ini_parser.hpp>

std::shared_ptr<soci::session> palm::sqlite3::open(
    const std::filesystem::path& file, const std::chrono::seconds& timeout,
    const bool wal_mode) {
  std::stringstream url;
  {
    url << "db=" << file.string();
    url << " timeout=" << timeout.count();
    url << " shared_cache=true";
    url << " synchronous=off";
  }
  std::shared_ptr<soci::session> it =
      std::make_shared<soci::session>(soci::sqlite3, url.str());
  it->set_logger(new palm::orm::Logger());

  (*it) << "PRAGMA foreign_keys = ON";
  if (wal_mode) {
    (*it) << "PRAGMA journal_mode = WAL";
  }

  return it;
}

std::shared_ptr<soci::session> palm::postgresql::open(
    const std::string& name, const std::string& host, const uint16_t port,
    const std::string& user, const std::optional<std::string> password) {
  // https://www.postgresql.org/docs/14/libpq-connect.html#LIBPQ-CONNSTRING
  std::stringstream url;
  {
    url << "host=" << host;
    url << " port=" << port;
    url << " dbname=" << name;
    url << " user=" << user;
    if (password) {
      url << " password=" << password.value();
    }
    url << " sslmode=disable";
    url << " connect_timeout=" << 12;
  }
  std::shared_ptr<soci::session> it =
      std::make_shared<soci::session>(soci::postgresql, url.str());
  it->set_logger(new palm::orm::Logger());
  return it;
}

void palm::orm::Query::load(const std::filesystem::path& root) {
  const std::lock_guard<std::mutex> lock(this->locker);
  for (const auto& it : std::filesystem::directory_iterator(root / "queries")) {
    const std::string file = it.path().string();
    BOOST_LOG_TRIVIAL(info) << "load query file from " << file;
    boost::property_tree::ptree tree;
    boost::property_tree::read_ini(file, tree);
    this->trees[file] = tree;
  }
}

palm::orm::Schema::Schema(const std::filesystem::path& root,
                          std::shared_ptr<soci::session> db)
    : db(db) {
  const auto top = root / "migrations";

  (*db) << palm::file2str(top / "initial-setup.sql");

  const auto& query = palm::orm::Query::instance();
  const std::string sql_by_version = query.get("schema-migrations.by-version");
  const std::string sql_create = query.get("schema-migrations.create");
  for (const auto& it : std::filesystem::directory_iterator(top)) {
    const auto node = it.path();
    if (!std::filesystem::is_directory(node)) {
      continue;
    }
    BOOST_LOG_TRIVIAL(info) << "load migration files from " << node;

    Migration mig;
    {
      std::string fn = node.filename();
      const size_t POS = 14;
      mig.version = fn.substr(0, POS);
      mig.name = fn.substr(POS + 1);
    }
    mig.up = palm::file2str(node / "up.sql");
    mig.down = palm::file2str(node / "down.sql");

    {
      std::optional<Migration> cur;

      (*db) << sql_by_version, soci::use(mig), soci::into(cur);

      if (cur) {
        if (cur->name != mig.name || cur->up != mig.up ||
            cur->down != mig.down) {
          std::stringstream ss;
          ss << "bad migration record " << cur->version;
          throw std::runtime_error(ss.str());
        }
      } else {
        BOOST_LOG_TRIVIAL(warning) << "can't found, save it";
        (*db) << sql_create, soci::use(mig.version, "version"),
            soci::use(mig.name, "name"), soci::use(mig.up, "up"),
            soci::use(mig.down, "down");
      }
    }
  }
}

void palm::orm::Schema::migrate() {
  const palm::orm::Query& query = palm::orm::Query::instance();
  const std::string sql_all = query.get("schema-migrations.all");
  const std::string sql_set_run_on = query.get("schema-migrations.migrate");
  soci::rowset<palm::orm::Migration> rows = (this->db->prepare << sql_all);

  for (const auto& it : rows) {
    if (it.run_on) {
      continue;
    }
    {
      BOOST_LOG_TRIVIAL(info) << "run migration " << it;
      soci::transaction tr(*this->db);
      (*this->db) << it.up;
      (*this->db) << sql_set_run_on, soci::use(it);
      tr.commit();
    }
  }
}

void palm::orm::Schema::rollback() {
  const palm::orm::Query& query = palm::orm::Query::instance();

  std::optional<Migration> cur;

  (*this->db) << query.get("schema-migrations.latest"), soci::into(cur);
  if (cur) {
    BOOST_LOG_TRIVIAL(info) << "rollback migration " << (*cur);
    soci::transaction tr(*this->db);
    (*this->db) << cur->down;
    (*this->db) << query.get("schema-migrations.rollback"), soci::use(cur);
    tr.commit();
  } else {
    BOOST_LOG_TRIVIAL(info) << "database is empty";
  }
}

void palm::orm::Schema::status(std::ostream& out) {
  const palm::orm::Query& query = palm::orm::Query::instance();
  const std::string sql_all = query.get("schema-migrations.all");

  soci::rowset<palm::orm::Migration> rows = (this->db->prepare << sql_all);
  const auto flags = out.flags();
  out << std::setiosflags(std::ios::left);
  for (const auto& it : rows) {
    out << it.version << " " << std::setw(32) << it.name << " ";
    if (it.run_on) {
      out << std::asctime(&it.run_on.value());
    } else {
      out << "Pending" << std::endl;
    }
  }
  out << std::setiosflags(flags);
}
