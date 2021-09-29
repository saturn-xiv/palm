// #pragma once

// #include "palm/env.hpp"

// #include <boost/asio.hpp>
// #include <boost/asio/serial_port.hpp>

// #ifndef PALM_SERIAL_PORT_READ_BUFFER_SIZE
// #define PALM_SERIAL_PORT_READ_BUFFER_SIZE (1 << 8)
// #endif

// #ifndef PALM_SERIAL_PORT_MAX_BUFFER_SIZE
// #define PALM_SERIAL_PORT_MAX_BUFFER_SIZE (1 << 16)
// #endif

// namespace palm {

// namespace tty {

// class Uuid {
//  public:
//   Uuid();

//   inline static Uuid zero() { return Uuid(0); }
//   inline static Uuid null() { return Uuid(0xffff); }
//   friend std::ostream& operator<<(std::ostream& out, const Uuid& self) {
//     std::ios_base::fmtflags f(out.flags());
//     out << std::setfill('0') << std::setw(4) << std::hex << self.id;
//     out.flags(f);
//     return out;
//   }

//  private:
//   Uuid(uint32_t id) : id(id) {}
//   inline std::string str(uint32_t id) {
//     std::stringstream ss;
//     return ss.str();
//   }

//   uint32_t id;
// };

// //
// https://blog.mbedded.ninja/programming/operating-systems/linux/linux-serial-ports-using-c-cpp/#full-example-standard-baud-rates
// class SerialPort {
//  public:
//   SerialPort(const std::string& name, uint16_t baud_rate = 9600);
//   ~SerialPort();

//   size_t write(const std::string& command);

//   inline static const std::string ORAGNTE_PI_UART1 = "/dev/ttyS1";
//   inline static const std::string ORAGNTE_PI_UART2 = "/dev/ttyS2";
//   inline static const std::string USB0 = "/dev/ttyUSB0";
//   inline static const std::string USB1 = "/dev/ttyUSB1";
//   inline static const std::string RASPBERRY_PI_UART1 = "/dev/serial0";

//  protected:
//   std::optional<std::pair<size_t, size_t>> match(
//       const std::string& buffer,
//       const std::vector<std::pair<std::string, std::string>>& items) const;
//   virtual std::optional<std::pair<size_t, size_t>> match(
//       const std::string& buffer) const = 0;
//   virtual void on_receive(const std::string& message) = 0;

//  private:
//   void read();
//   void on_receive(const boost::system::error_code& ec,
//                   size_t bytes_transferred);

//   std::mutex locker;
//   std::string buffer;
//   char line[PALM_SERIAL_PORT_READ_BUFFER_SIZE];
//   boost::asio::io_service io_service;
//   std::shared_ptr<boost::asio::serial_port> port;
// };
// }  // namespace tty

// }  // namespace palm
