#define BOOST_TEST_MODULE validator
#include <boost/test/included/unit_test.hpp>

#include "basil/queue.hpp"
#include "basil/validator.hpp"

#include <boost/type_index.hpp>

BOOST_AUTO_TEST_CASE(fields) { BOOST_CHECK_EQUAL(1 + 1, 2); }

BOOST_AUTO_TEST_CASE(types) {
  std::cout << "size_t: " << boost::typeindex::type_id<size_t>().pretty_name()
            << std::endl;
  std::cout << "int8_t: " << boost::typeindex::type_id<int8_t>().pretty_name()
            << std::endl;
  std::cout << "uint8_t: " << boost::typeindex::type_id<uint8_t>().pretty_name()
            << std::endl;
  std::cout << "STD String: "
            << boost::typeindex::type_id<std::string>().pretty_name()
            << std::endl;
  std::cout
      << "RabbitMQ Client: "
      << boost::typeindex::type_id<basil::rabbitmq::Client>().pretty_name()
      << std::endl;
  std::cout
      << "RabbitMQ Client(*): "
      << boost::typeindex::type_id<basil::rabbitmq::Client*>().pretty_name()
      << std::endl;
  std::cout
      << "RabbitMQ Client(&): "
      << boost::typeindex::type_id<basil::rabbitmq::Client&>().pretty_name()
      << std::endl;
}
