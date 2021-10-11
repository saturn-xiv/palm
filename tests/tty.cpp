#define BOOST_TEST_MODULE theme
#include <boost/test/unit_test.hpp>

#include "palm/tty.hpp"

#include <boost/algorithm/string.hpp>

void write_f(std::shared_ptr<palm::Tty> port) {
  std::ifstream file(std::getenv("TTY_SCRIPT"));
  for (std::string line; std::getline(file, line);) {
    boost::trim(line);
    if (line.size() > 0) {
      std::cout << "read line: " << line << std::endl;
      port->write(line);
      std::this_thread::sleep_for(std::chrono::seconds(1));
    }
  }
}

void read_f(std::shared_ptr<palm::Tty> tty) {
  for (auto i = 1; i < 100; i++) {
    const auto it = tty->read();
    std::cout << "receive message(" << i << "): " << it << std::endl;
  }
}

BOOST_AUTO_TEST_CASE(rs232) {
  std::shared_ptr<palm::Tty> port =
      std::make_shared<palm::Tty>(std::getenv("TTY_NAME"));
  std::thread tw(write_f, port);
  std::thread tr(read_f, port);
  tw.join();
  tr.join();
}
