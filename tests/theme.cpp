#include <catch2/catch_test_macros.hpp>

#include "palm/theme.hpp"

#include <iostream>

TEST_CASE("pagination", "[theme]") {
  {
    const auto [index, size] = palm::paginate(100, -1, -1);
    REQUIRE(index == 1);
    REQUIRE(size == 4096);
  }
  {
    const auto [index, size] = palm::paginate(100, 0, 0);
    REQUIRE(index == 1);
    REQUIRE(size == 10);
  }
  {
    const auto [index, size] = palm::paginate(100, 0, 6);
    REQUIRE(index == 1);
    REQUIRE(size == 10);
  }
  {
    const auto [index, size] = palm::paginate(100, -1, (1 << 12) + 1);
    REQUIRE(index == 1);
    REQUIRE(size == (1 << 12));
  }
  {
    const auto [index, size] = palm::paginate(100, 2, 200);
    REQUIRE(index == 1);
    REQUIRE(size == 200);
  }
  {
    const auto [index, size] = palm::paginate(100, 2, 20);
    REQUIRE(index == 2);
    REQUIRE(size == 20);
  }
  {
    const auto [index, size] = palm::paginate(100, 20, 20);
    REQUIRE(index == 5);
    REQUIRE(size == 20);
  }
  {
    const auto [index, size] = palm::paginate(101, 20, 20);
    REQUIRE(index == 6);
    REQUIRE(size == 20);
  }
}
