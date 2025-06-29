#define BOOST_TEST_MODULE cache
#include <boost/test/included/unit_test.hpp>

#include "basil/cache.hpp"

BOOST_AUTO_TEST_CASE(redis) {
  basil::redis::Node config;
  auto pool = config.open();
  {
    const auto info = pool->info();
    std::cout << info << std::endl;
  }

  const std::string hi = "Hello, basil!";
  {
    const std::string key = "hi";
    pool->setex(key, std::chrono::minutes(1), hi);
    {
      const auto val = pool->get(key);
      BOOST_REQUIRE(val.has_value());
      BOOST_CHECK_EQUAL(val.value(), hi);
    }
  }
}
