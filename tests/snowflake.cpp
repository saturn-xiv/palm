#define BOOST_TEST_MODULE snowflake
#include <boost/test/unit_test.hpp>

#include "palm/snowflake.hpp"

#include <iostream>

BOOST_AUTO_TEST_CASE(random_id) {
  palm::Snowflake& it = palm::Snowflake::instance();
  //  FIXME chrono
  // it.set(std::chrono::year_month_day(std::chrono::December / 21 / 2021), 8,
  // 8);
  for (int i = 0; i < 100; i++) {
    std::cout << i << ": " << it.next() << std::endl;
  }
}
