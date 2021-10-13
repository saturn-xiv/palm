#define BOOST_TEST_MODULE orm
#include <boost/test/unit_test.hpp>

#include "palm/orm.hpp"

BOOST_AUTO_TEST_CASE(sqlite) { palm::sqlite::open("db"); }

BOOST_AUTO_TEST_CASE(postgresql) {
  const palm::postgresql::Config cfg("demo");
  auto con = cfg.open();
}
