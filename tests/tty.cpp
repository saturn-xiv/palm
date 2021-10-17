#define BOOST_TEST_MODULE theme
#include <boost/test/unit_test.hpp>

#include "palm/tty.hpp"

#include <boost/algorithm/string.hpp>

class SerialPort : public palm::SerialPort {
 protected:
  void on_receive(const std::string& message) const override {
    std::cout << "handle message: " << message << std::endl;
  }
  std::optional<std::pair<size_t, size_t>> match(
      const std::string& buffer) const override {
    return palm::SerialPort::match(buffer, "[", "]");
  }
};

BOOST_AUTO_TEST_CASE(rs232) {
  std::shared_ptr<SerialPort> port = std::make_shared<SerialPort>();
  port->start(std::getenv("TTY_NAME"));
  std::ifstream file(std::getenv("TTY_SCRIPT"));
  for (std::string line; std::getline(file, line);) {
    boost::trim(line);
    if (line.size() > 0) {
      std::cout << "read line: " << line << std::endl;
      port->write(line);
      std::this_thread::sleep_for(std::chrono::minutes(1));
    }
  }
}
