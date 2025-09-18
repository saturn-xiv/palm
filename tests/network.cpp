#include <catch2/catch_test_macros.hpp>

#include "palm/network.hpp"

#include <iostream>

TEST_CASE("ipv4", "[cidr]") {
  palm::network::Ipv4 it("", "");
  for (uint8_t i = 1; i <= 32; i++) {
    it.cidr(i);
    std::cout << "CIDR/" << std::to_string(i)
              << "\tto netmask: " << it.netmask() << std::endl;
    { REQUIRE(it.cidr() == i); }
  }
}
