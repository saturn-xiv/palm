#include <catch2/catch_test_macros.hpp>

#include "palm/network.hpp"

#include <iostream>

TEST_CASE("network", "[ipv4]") {
  SECTION("netmask") {
    palm::network::Ipv4 it("192.168.0.10", "255.255.255.0");
    for (uint8_t i = 1; i <= 32; i++) {
      it.cidr(i);
      std::cout << "CIDR/" << std::to_string(i)
                << "\tto netmask: " << it.netmask() << std::endl;
      REQUIRE(it.cidr() == i);
    }
  }

  SECTION("broadcast") {
    palm::network::Ipv4 it("192.168.1.10", "255.255.255.0");

    REQUIRE(it.broadcast() == "192.168.1.255");
    REQUIRE(it.network() == "192.168.1.0");
    REQUIRE(it.default_gateway() == "192.168.1.1");

    {
      const auto ips = it.addresses();
      REQUIRE(ips.size() > 2);
      REQUIRE(ips.front() == "192.168.1.2");
      REQUIRE(ips.back() == "192.168.1.254");
    }
  }
}
