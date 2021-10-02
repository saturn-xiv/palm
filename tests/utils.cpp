#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

#include "palm/utils.hpp"

TEST_CASE("system", "[utils]") {
  std::cout << "current user: " << palm::current_user() << std::endl;
  {
    const auto u = palm::os();
    std::cout << "sysname : " << u.sysname << '\n'
              << "nodename: " << u.nodename << '\n'
              << "release : " << u.release << '\n'
              << "version : " << u.version << '\n'
              << "machine : " << u.machine << std::endl;
  }
}
