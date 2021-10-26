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
  std::shared_ptr<palm::orm::Query> query =
      std::make_shared<palm::orm::Query>(root);
  std::cout << "get query: " << query->get("schema_migrations.all-asc")
            << std::endl;

  palm::postgresql::Config cfg("palm_test");
  auto pool = palm::orm::pool::open(soci::postgresql, cfg.url(), 12);
  {
    std::shared_ptr<soci::session> sql = std::make_shared<soci::session>(*pool);
    std::tm now;
    *sql << query->get("schema_migrations.heartbeat"), soci::into(now);
    std::cout << "current db timestamp: " << std::asctime(&now) << std::endl;
  }

  {
    std::shared_ptr<soci::session> sql = std::make_shared<soci::session>(*pool);
    palm::orm::migration::Migration mig(sql, query, root);
    mig.migrate();
    mig.rollback();
  }
  {
    std::shared_ptr<soci::session> sql = std::make_shared<soci::session>(*pool);
    palm::orm::migration::Migration mig(sql, query, root);
    mig.status(std::cout);
  }
}
