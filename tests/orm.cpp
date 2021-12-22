#define BOOST_TEST_MODULE orm
#include <boost/test/unit_test.hpp>

#include "palm/orm.hpp"

#define DB_ROOT "db/postgresql"

BOOST_AUTO_TEST_CASE(query) {
  palm::orm::Query& it = palm::orm::Query::instance();
  it.load(DB_ROOT);

  for (auto const& k :
       {"schema-migrations.all", "schema-migrations.by-version"}) {
    std::cout << k << " = " << it.get(k) << std::endl;
  }
}

BOOST_AUTO_TEST_CASE(sqlite3) {
  auto db = palm::sqlite3::open("test.db");
  std::string version;
  (*db) << "SELECT SQLITE_VERSION()", soci::into(version);
  std::cout << "sqlite3 version: " << version << std::endl;
}

BOOST_AUTO_TEST_CASE(postgresql) {
  auto db = palm::postgresql::open("demo");
  {
    std::string version;
    (*db) << "SELECT VERSION()", soci::into(version);
    std::cout << "postgresql version: " << version << std::endl;
  }

  {
    palm::orm::Schema it = palm::orm::Schema(DB_ROOT, db);

    it.status(std::cout);
    it.migrate();
    it.status(std::cout);
    it.rollback();
    it.status(std::cout);
  }
}
