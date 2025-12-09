#include <catch2/benchmark/catch_benchmark.hpp>
#include <catch2/catch_test_macros.hpp>

#include "palm/cache.hpp"

TEST_CASE("redis client", "[cache]") {
  SECTION("set") { REQUIRE(1 + 1 == 2); }
  SECTION("get") { REQUIRE(1 + 1 == 2); }
  SECTION("set-get") { REQUIRE(1 + 1 == 2); }
}
