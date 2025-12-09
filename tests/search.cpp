#include <catch2/benchmark/catch_benchmark.hpp>
#include <catch2/catch_test_macros.hpp>

#include "palm/search.hpp"

TEST_CASE("indes", "[opensearch]") {
  SECTION("create") { REQUIRE(1 + 1 == 2); }
  SECTION("list") { REQUIRE(1 + 1 == 2); }
}
