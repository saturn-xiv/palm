#define BOOST_TEST_MODULE snowflake
#include <boost/test/unit_test.hpp>

#include <iostream>
#include "palm/snowflake.hpp"

BOOST_AUTO_TEST_CASE(random_id) {
  palm::Snowflake& it = palm::Snowflake::get();
  it.set(8, 8);
  for (int i = 0; i < 100; i++) {
    std::cout << i << ": " << it.next() << std::endl;
  }
}
