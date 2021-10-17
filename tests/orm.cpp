#define BOOST_TEST_MODULE orm
#include <boost/test/unit_test.hpp>

#include "palm/crypto.hpp"
#include "palm/orm.hpp"

BOOST_AUTO_TEST_CASE(sqlite) { palm::sqlite::open("db.sqlite3"); }

BOOST_AUTO_TEST_CASE(postgresql) {
  const palm::postgresql::Config cfg("demo");
  auto con = cfg.open();
}

BOOST_AUTO_TEST_CASE(migration) {
  const std::filesystem::path root = "db-test";

  const std::string name = palm::random::alphanumeric(8);
  palm::orm::migration::Item::generate(root, name);
}

BOOST_AUTO_TEST_CASE(schema) {
  const std::filesystem::path root = "db/postgresql";
  palm::orm::Query::load(root);
  std::cout << "get query: "
            << palm::orm::Query::get("schema_migrations.all-asc") << std::endl;

  palm::postgresql::Config cfg("palm_test");
  palm::orm::Pool::open(soci::postgresql, cfg.url(), 12);
  {
    auto sql = palm::orm::Pool::get();
    std::tm now;
    *sql << palm::orm::Query::get("schema_migrations.heartbeat"),
        soci::into(now);
    std::cout << "current db timestamp: " << std::asctime(&now) << std::endl;
  }

  {
    auto sql = palm::orm::Pool::get();
    palm::orm::migration::load(*sql, root);
    palm::orm::migration::migrate(*sql);
    palm::orm::migration::rollback(*sql);
    palm::orm::migration::status(*sql, std::cout);
  }
}
