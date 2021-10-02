#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

#include "palm/utils.hpp"

TEST_CASE("random string", "[utils]") {
  for (auto i = 0; i < 10; i++) {
    std::cout << palm::timestamp() << " " << palm::random::string(8) << " "
              << palm::uuid() << std::endl;
  }
}

TEST_CASE("random bytes", "[utils]") {
  const size_t len = 32;
  for (auto i = 0; i < 10; i++) {
    const auto d1 = palm::random::bytes(len);
    REQUIRE(d1.size() == len);
    // const unsigned char* buffer = (data.size() ? &data.front() : nullptr);
    const auto s1 = palm::hex::to(d1);
    REQUIRE(s1.size() == len * 2);
    std::cout << s1 << std::endl;
    const auto d2 = palm::hex::from(s1);
    const auto s2 = palm::hex::to(d2);
    REQUIRE(s1 == s2);
    REQUIRE(d1.size() == d2.size());
    for (auto i = 0; i < len; i++) {
      REQUIRE(d1[i] == d2[i]);
    }
  }
}
