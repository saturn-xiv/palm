#include <catch2/benchmark/catch_benchmark.hpp>
#include <catch2/catch_test_macros.hpp>

#include "palm/portal.hpp"

TEST_CASE("grpc client", "rpc]") {
  SECTION("client") { REQUIRE(1 + 1 == 2); }
  SECTION("health-check") { REQUIRE(1 + 1 == 2); }
}
