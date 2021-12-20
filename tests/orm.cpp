#define BOOST_TEST_MODULE rbac
#include <boost/test/unit_test.hpp>

#include "palm/orm.hpp"

BOOST_AUTO_TEST_CASE(query) {
  palm::orm::Query& it = palm::orm::Query::instance();
  it.load("db/postgresql");

  for (auto const& k : {"schema-migration.up", "schema-migration.down"}) {
    std::cout << k << " = " << it.get(k).value() << std::endl;
  }
}
