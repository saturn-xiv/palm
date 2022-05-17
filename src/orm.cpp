#include "palm/orm.hpp"

#include <boost/date_time/gregorian/formatters.hpp>
#include <boost/log/trivial.hpp>
#include <boost/property_tree/ini_parser.hpp>

#include <Poco/Data/MySQL/Connector.h>
#include <Poco/Data/PostgreSQL/Connector.h>
#include <Poco/Data/Transaction.h>

std::string palm::orm::Schema::latest() const {
  return R"SQL(SELECT version, name, up, down, run_on, created_at FROM schema_migrations WHERE run_on IS NOT NULL ORDER BY version DESC)SQL";
}
std::string palm::orm::Schema::down() const {
  return R"SQL(UPDATE schema_migrations SET run_on = NULL WHERE version = $1)SQL";
}
std::string palm::orm::Schema::up() const {
  return R"SQL(UPDATE schema_migrations SET run_on = CURRENT_TIMESTAMP WHERE version = $1)SQL";
}
std::string palm::orm::Schema::select() const {
  return R"SQL(SELECT version, name, up, down, run_on, created_at FROM schema_migrations ORDER BY version ASC)SQL";
}
std::string palm::orm::Schema::insert() const {
  return R"SQL(INSERT INTO schema_migrations(version, name, up, down) VALUES($1, $2, $3, $4))SQL";
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
    boost::property_tree::read_ini(file, tree);
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
