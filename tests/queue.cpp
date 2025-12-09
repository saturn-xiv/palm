#include <catch2/benchmark/catch_benchmark.hpp>
#include <catch2/catch_test_macros.hpp>

#include "palm/queue.hpp"

TEST_CASE("c", "[rabbitmq]") {
  SECTION("product") { REQUIRE(1 + 1 == 2); }
  SECTION("consume") { REQUIRE(1 + 1 == 2); }
  SECTION("publish") { REQUIRE(1 + 1 == 2); }
  SECTION("subscribe") { REQUIRE(1 + 1 == 2); }
}
