#define BOOST_TEST_MODULE tty
#include <boost/test/included/unit_test.hpp>

// #include "palm/tty.hpp"

// #include <unistd.h>
// #include <cstdlib>
// #include <fstream>
// #include <iostream>
// #include <thread>

BOOST_AUTO_TEST_CASE(uuid) {
  // {
  //   std::stringstream ss;
  //   ss << palm::tty::Uuid::zero();
  //   BOOST_CHECK_EQUAL(ss.str(), "0000");
  // }
  // {
  //   std::stringstream ss;
  //   ss << palm::tty::Uuid::null();
  //   BOOST_CHECK_EQUAL(ss.str(), "ffff");
  // }

  // for (int i = 1; i < 100; i++) {
  //   std::cout << i << "\t" << palm::tty::Uuid() << std::endl;
  // }

  // {
  //   std::stringstream ss;
  //   ss << palm::tty::Uuid::zero();
  //   ss << 22;

  //   BOOST_CHECK_EQUAL(ss.str(), "000022");
  // }
}

// class SerialPort : public palm::tty::SerialPort {
//  public:
//   SerialPort() : palm::tty::SerialPort(palm::tty::SerialPort::USB0) {}

//  protected:
//   void on_receive(const std::string& message) override {
//     std::cout << "handle message: " << message << std::endl;
//   }
//   std::optional<std::pair<size_t, size_t>> match(
//       const std::string& buffer) const override {
//     std::vector<std::pair<std::string, std::string>> items{
//         std::make_pair("[", "]"), std::make_pair("<", ">")};
//     return this->palm::tty::SerialPort::match(buffer, items);
//   }
// };

BOOST_AUTO_TEST_CASE(read_loop) {
  // SerialPort tty;

  // for (int i = 0; i < 60 * 24; i++) {
  //   ::usleep(60 * 1000);
  // }
}
