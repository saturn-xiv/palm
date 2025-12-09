#include <catch2/benchmark/catch_benchmark.hpp>
#include <catch2/catch_test_macros.hpp>

#include "palm/orm.hpp"

TEST_CASE("pqxx", "[postgresql]") {
  SECTION("date") { REQUIRE(1 + 1 == 2); }
  SECTION("pool") { REQUIRE(1 + 1 == 2); }
}
