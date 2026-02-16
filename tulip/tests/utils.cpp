#include <catch2/benchmark/catch_benchmark.hpp>
#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_vector.hpp>

#include "palm/snowflake.hpp"
#include "palm/utils.hpp"

TEST_CASE("random", "[utils]") {
  SECTION("bytes") {
    const auto l = 16;
    const auto v1 = palm::random::bytes(l);
    const auto v2 = palm::random::bytes(l);
    REQUIRE(v1.size() == l);
    REQUIRE(v1.size() == v2.size());
    REQUIRE_THAT(v1, !Catch::Matchers::Equals(v2));
    std::cout << "Random bytes: " << palm::base64::to(v1) << ", "
              << palm::base64::to(v2) << std::endl;
  }
  SECTION("uuid") {
    const auto v1 = palm::uuid();
    const auto v2 = palm::uuid();
    REQUIRE(v1 != v2);

    std::cout << "UUID: " << v1 << ", " << v2 << std::endl;
  }
  SECTION("alphanumeric") {
    const auto l = 16;
    const auto v1 = palm::random::alphanumeric(l);
    const auto v2 = palm::random::alphanumeric(l);
    REQUIRE(v1 != v2);

    std::cout << "Random alphanumeric string: " << v1 << ", " << v2
              << std::endl;
  }
}

TEST_CASE("Generate snowflake ids", "[snowflake]") {
  std::cout << "Current epoch in milliseconds: "
            << std::chrono::duration_cast<std::chrono::milliseconds>(
                   std::chrono::system_clock::now().time_since_epoch())
                   .count()
            << std::endl;

  SECTION("id") {
    palm::Snowflake sf1(1);
    palm::Snowflake sf2(2);

    const auto v11 = sf1.next();
    const auto v12 = sf1.next();
    const auto v21 = sf2.next();

    REQUIRE(v11 + 1 == v12);
    REQUIRE(v11 < v21);

    std::cout << "Node-1 ID: " << v11 << ", " << v12 << std::endl;
    std::cout << "Node-2 ID: " << v21 << std::endl;
  }
}
