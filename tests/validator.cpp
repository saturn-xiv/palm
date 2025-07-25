#include <catch2/catch_test_macros.hpp>

#include "palm/queue.hpp"
#include "palm/utils.hpp"
#include "palm/validator.hpp"

#include <iostream>

TEST_CASE("check types", "[types]") {
  std::cout << "size_t: " << boost::typeindex::type_id<size_t>().pretty_name()
            << std::endl;
  std::cout << "int8_t: " << boost::typeindex::type_id<int8_t>().pretty_name()
            << std::endl;
  std::cout << "uint8_t: " << boost::typeindex::type_id<uint8_t>().pretty_name()
            << std::endl;
  std::cout << "STD String: "
            << boost::typeindex::type_id<std::string>().pretty_name()
            << std::endl;
  std::cout << "RabbitMQ Client: "
            << boost::typeindex::type_id<palm::rabbitmq::Client>().pretty_name()
            << std::endl;
  std::cout
      << "RabbitMQ Client(*): "
      << boost::typeindex::type_id<palm::rabbitmq::Client*>().pretty_name()
      << std::endl;
  std::cout
      << "RabbitMQ Client(&): "
      << boost::typeindex::type_id<palm::rabbitmq::Client&>().pretty_name()
      << std::endl;
}

TEST_CASE("boost process2", "[shell]") {
  spdlog::set_level(spdlog::level::debug);
  SECTION("stdout") {
    const auto& [code, out, err] = palm::shell("/usr/bin/podman", {"ps", "-a"});
    std::cout << "exit code: " << code << std::endl;
    std::cout << "STDOUT: " << out << std::endl;
    std::cout << "STDERR: " << err << std::endl;
    REQUIRE(!out.empty());
  }
  SECTION("stderr") {
    const auto& [code, out, err] = palm::shell("/usr/bin/ls", {"-a", "/aaa"});
    std::cout << "exit code: " << code << std::endl;
    std::cout << "STDOUT: " << out << std::endl;
    std::cout << "STDERR: " << err << std::endl;
    REQUIRE(!err.empty());
  }
}

TEST_CASE("std tm", "[datetime]") {
  SECTION("time_t to *tm") {
    for (const time_t i : {1753369286, 1753369586}) {
      std::tm* t = std::localtime(&i);
      std::cout << i << " " << std::ctime(&i) << std::asctime(t);
    }
  }
}

TEST_CASE("booted at", "[datetime]") {
  auto i = palm::booted_at();
  REQUIRE(i.has_value());

  {
    auto it = i.value();
    std::cout << "BOOTED AT: " << std::ctime(&it);
  }
}
