#pragma once

#include "palm/env.hpp"

#include <boost/asio.hpp>
#include <boost/asio/serial_port.hpp>
#include <boost/bind/bind.hpp>
#include <boost/system/error_code.hpp>
#include <boost/system/system_error.hpp>
#include <boost/thread.hpp>

#ifndef PALM_SERIAL_PORT_READ_BUFFER_SIZE
#define PALM_SERIAL_PORT_READ_BUFFER_SIZE (1 << 8)
#endif

namespace palm {

class SerialPort {
 public:
  SerialPort() {}
  ~SerialPort() { this->stop(); }

  void start(
      const std::string& name,
      const boost::asio::serial_port_base::baud_rate& baud_rate =
          boost::asio::serial_port_base::baud_rate(9600),
      const boost::asio::serial_port_base::character_size& character_size =
          boost::asio::serial_port_base::character_size(8),
      const boost::asio::serial_port_base::stop_bits& stop_bits =
          boost::asio::serial_port_base::stop_bits(
              boost::asio::serial_port_base::stop_bits::one),
      const boost::asio::serial_port_base::parity& parity =
          boost::asio::serial_port_base::parity(
              boost::asio::serial_port_base::parity::none),
      const boost::asio::serial_port_base::flow_control& flow_control =
          boost::asio::serial_port_base::flow_control(
              boost::asio::serial_port_base::flow_control::none));
  void stop();
  int write(const std::string& message);
  int write(const char* buffer, const int& size);

 protected:
  std::optional<std::pair<size_t, size_t>> match(
      const std::string& buffer, const std::string& header,
      const std::string& footer) const;
  virtual void on_receive(const std::string& message) const = 0;
  virtual std::optional<std::pair<size_t, size_t>> match(
      const std::string& buffer) const = 0;

 private:
  void async_read();
  void on_receive(const boost::system::error_code& ec,
                  const size_t bytes_transferred);

  std::mutex mutex;
  boost::asio::io_service io_service;
  std::shared_ptr<boost::asio::serial_port> port;
  char line[PALM_SERIAL_PORT_READ_BUFFER_SIZE + 1];
  std::string buffer;
};

}  // namespace palm
