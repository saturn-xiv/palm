#include <catch2/catch_test_macros.hpp>

#include "palm/captcha.hpp"

#include <fstream>
#include <iostream>

#include <spdlog/spdlog.h>

TEST_CASE("by png", "[captcha]") {
  spdlog::set_level(spdlog::level::debug);

  const std::string hi = "Hello, palm!";

  {
    const auto buf = palm::captcha::png(hi, 32);
    std::cout << "generate '" << hi << "' " << buf.size() << "bytes"
              << std::endl;

    std::ofstream out("hello.png", std::ios::out | std::ios::binary);
    REQUIRE(out.is_open());
    out.write(reinterpret_cast<const char*>(buf.data()), buf.size());
    out.close();
  }
}
