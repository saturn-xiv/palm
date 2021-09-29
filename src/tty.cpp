// #include "palm/tty.hpp"

// #include <cstring>
// #include <exception>

// #include <boost/bind.hpp>
// #include <boost/log/trivial.hpp>
// #include <boost/system/error_code.hpp>
// #include <boost/system/system_error.hpp>
// #include <boost/thread.hpp>

// #include "palm/queue.hpp"

// palm::tty::Uuid::Uuid() {
//   static std::mutex locker;
//   static uint32_t cur;

//   {
//     std::lock_guard<std::mutex> guard(locker);
//     cur++;
//     if (cur >= 0xffff) {
//       cur = 1;
//     }
//     this->id = cur;
//   }
// }

// palm::tty::SerialPort::SerialPort(const std::string& name, uint16_t
// baud_rate) {
//   this->port = std::make_shared<boost::asio::serial_port>(this->io_service);
//   BOOST_LOG_TRIVIAL(info) << "open serial port " << name;
//   this->port->open(name);
//   this->port->set_option(boost::asio::serial_port_base::baud_rate(baud_rate));
//   this->port->set_option(boost::asio::serial_port_base::character_size(8));
//   this->port->set_option(boost::asio::serial_port_base::stop_bits(
//       boost::asio::serial_port_base::stop_bits::one));
//   this->port->set_option(boost::asio::serial_port_base::parity(
//       boost::asio::serial_port_base::parity::none));
//   this->port->set_option(boost::asio::serial_port_base::flow_control(
//       boost::asio::serial_port_base::flow_control::none));

//   boost::thread t(
//       boost::bind(&boost::asio::io_service::run, &this->io_service));

//   this->read();
// }
// palm::tty::SerialPort::~SerialPort() {
//   this->port->cancel();
//   this->port->close();

//   this->io_service.stop();
//   this->io_service.reset();
// }

// size_t palm::tty::SerialPort::write(const std::string& command) {
//   if (command.empty()) {
//     return 0;
//   }
//   BOOST_LOG_TRIVIAL(info) << "write " << command;
//   return this->port->write_some(
//       boost::asio::buffer(command.c_str(), command.size()));
// }

// void palm::tty::SerialPort::read() {
//   std::memset(this->line, 0, PALM_SERIAL_PORT_READ_BUFFER_SIZE);

//   this->port->async_read_some(
//       boost::asio::buffer(this->line, PALM_SERIAL_PORT_READ_BUFFER_SIZE - 1),
//       boost::bind(&palm::tty::SerialPort::on_receive, this,
//                   boost::asio::placeholders::error,
//                   boost::asio::placeholders::bytes_transferred));
// }

// void palm::tty::SerialPort::on_receive(const boost::system::error_code& ec,
//                                        size_t bytes_transferred) {
//   if (ec) {
//     BOOST_LOG_TRIVIAL(error) << ec.message();
//     this->read();
//     return;
//   }

//   std::lock_guard<std::mutex> guard(this->locker);
//   std::string it(this->line);
//   BOOST_LOG_TRIVIAL(info) << "receive " << bytes_transferred
//                           << " bytes: " << it;
//   this->buffer += it;

//   for (;;) {
//     BOOST_LOG_TRIVIAL(info) << "current buffer: " << this->buffer;
//     const auto matcher = this->match(this->buffer);
//     if (!matcher) {
//       break;
//     }

//     const auto it = matcher.value();
//     const auto message = this->buffer.substr(it.first, it.second - it.first);
//     BOOST_LOG_TRIVIAL(info) << "match: " << message;

//     this->on_receive(message);

//     if (it.first > 0) {
//       const auto begin = this->buffer.substr(0, it.first);
//       BOOST_LOG_TRIVIAL(warning) << "unknown: " << begin;
//     }
//     this->buffer = this->buffer.substr(it.second);
//     BOOST_LOG_TRIVIAL(info) << "remain: " << this->buffer;
//   }

//   if (buffer.size() >= PALM_SERIAL_PORT_MAX_BUFFER_SIZE) {
//     BOOST_LOG_TRIVIAL(warning) << "clear tty buffer: " << this->buffer;
//     buffer.clear();
//   }
//   this->read();
// }

// std::optional<std::pair<size_t, size_t>> palm::tty::SerialPort::match(
//     const std::string& buffer,
//     const std::vector<std::pair<std::string, std::string>>& items) const {
//   for (const auto& it : items) {
//     auto begin = buffer.find(it.first);
//     auto end = buffer.find(it.second);
//     if (begin != std::string::npos && end != std::string::npos) {
//       return std::make_pair(begin, end + it.second.size());
//     }
//   }
//   return std::nullopt;
// }
