#define BOOST_TEST_MODULE orm
#include <boost/test/unit_test.hpp>

#include "palm/orm.hpp"

BOOST_AUTO_TEST_CASE(sqlite) { palm::sqlite::open("db"); }
