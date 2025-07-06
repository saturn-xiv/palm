#include <catch2/catch_test_macros.hpp>

#include "palm/cache.hpp"

#include <iostream>

TEST_CASE("redis cache", "[redis]") {
  palm::redis::Node config;
  auto pool = config.open();
  {
    const auto info = pool->info();
    std::cout << info << std::endl;
  }

  const std::string hi = "Hello, palm!";
  {
    const std::string key = "hi";
    pool->setex(key, std::chrono::minutes(1), hi);
    {
      const auto val = pool->get(key);
      REQUIRE(val.has_value());
      REQUIRE(val.value() == hi);
    }
  }
}
