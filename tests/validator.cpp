#include <catch2/catch_test_macros.hpp>

#include "palm/queue.hpp"
#include "palm/validator.hpp"

#include <iostream>

#include <boost/type_index.hpp>

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
