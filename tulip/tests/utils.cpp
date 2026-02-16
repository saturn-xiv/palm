#include <catch2/benchmark/catch_benchmark.hpp>
#include <catch2/catch_test_macros.hpp>

#include "palm/snowflake.hpp"
#include "palm/utils.hpp"

TEST_CASE("random", "[utils]") {
  SECTION("bytes") { REQUIRE(1 + 1 == 2); }
  SECTION("uuid") { REQUIRE(1 + 1 == 2); }
  SECTION("alphanumeric") { REQUIRE(1 + 1 == 2); }
}

TEST_CASE("Generate snowflake ids", "[snowflake]") {}
