#define BOOST_TEST_MODULE orm
#include <boost/test/unit_test.hpp>

#include "palm/crypto.hpp"
#include "palm/orm.hpp"

BOOST_AUTO_TEST_CASE(sqlite) { palm::sqlite::open("db"); }

BOOST_AUTO_TEST_CASE(postgresql) {
  const palm::postgresql::Config cfg("demo");
  auto con = cfg.open();
}

BOOST_AUTO_TEST_CASE(migration) {
  const std::filesystem::path root = "db-test";
  {
    const std::string name = palm::random::alphanumeric(8);
    palm::orm::Migration::generate(root, name);
    const auto migrations = palm::orm::Schema::load_migrations(root);
    palm::orm::Migration::header(std::cout);
    std::cout << std::endl;
    for (const auto& it : migrations) {
      std::cout << it << std::endl;
    }
  }
  {
    const auto queries = palm::orm::Schema::load_queries(root);
    for (const auto& it : queries) {
      std::cout << it.first << "=" << it.second;
    }
  }
}
