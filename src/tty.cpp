#include "palm/tty.hpp"

void palm::SerialPort::start(
    const std::string& name,
    const boost::asio::serial_port_base::baud_rate& baud_rate,
    const boost::asio::serial_port_base::character_size& character_size,
    const boost::asio::serial_port_base::stop_bits& stop_bits,
    const boost::asio::serial_port_base::parity& parity,
    const boost::asio::serial_port_base::flow_control& flow_control) {
  BOOST_LOG_TRIVIAL(info) << "open serial port " << name;
  if (this->port) {
    std::stringstream ss;
    ss << "serial port " << name << " already opened";
    throw std::invalid_argument(ss.str());
  }

  this->port = std::make_shared<boost::asio::serial_port>(this->io_service);
  this->port->open(name);

  this->port->set_option(baud_rate);
  this->port->set_option(character_size);
  this->port->set_option(stop_bits);
  this->port->set_option(parity);
  this->port->set_option(flow_control);

  boost::thread thread(
      boost::bind(&boost::asio::io_service::run, &this->io_service));

  this->async_read();
}
void palm::SerialPort::stop() {
  std::lock_guard<std::mutex> lock(this->mutex);
  if (this->port) {
    this->port->cancel();
    this->port->close();
    this->port.reset();
  }
  this->io_service.stop();
  this->io_service.reset();
}
int palm::SerialPort::write(const std::string& message) {
  BOOST_LOG_TRIVIAL(info) << "write message: " << message;
  return this->write(message.c_str(), message.size());
}
int palm::SerialPort::write(const char* buffer, const int& size) {
  std::lock_guard<std::mutex> lock(this->mutex);
  if (buffer == nullptr) {
    BOOST_LOG_TRIVIAL(error) << "null message";
    return -1;
  }
  if (size == 0) {
    BOOST_LOG_TRIVIAL(warning) << "empty message";
    return 0;
  }
  const auto len = this->port->write_some(boost::asio::buffer(buffer, size));
  BOOST_LOG_TRIVIAL(debug) << "written " << len << " bytes";
  return len;
}

void palm::SerialPort::async_read() {
  if (!this->port || !this->port->is_open()) {
    BOOST_LOG_TRIVIAL(error) << "serial port havn't opened yet";
    return;
  }
  memset(&this->line[0], 0, sizeof(this->line));
  this->port->async_read_some(
      boost::asio::buffer(this->line, PALM_SERIAL_PORT_READ_BUFFER_SIZE),
      boost::bind(&SerialPort::on_receive, this,
                  boost::asio::placeholders::error,
                  boost::asio::placeholders::bytes_transferred));
}

void palm::SerialPort::on_receive(const boost::system::error_code& ec,
                                  const size_t bytes_transferred) {
  std::lock_guard<std::mutex> lock(this->mutex);

  if (!this->port || !this->port->is_open()) {
    BOOST_LOG_TRIVIAL(error) << "serial port havn't opened yet";
    return;
  };
  if (ec) {
    BOOST_LOG_TRIVIAL(error)
        << "failed on read from serial port: " << ec.message();
    this->async_read();
    return;
  }
  const std::string line(this->line, bytes_transferred);
  BOOST_LOG_TRIVIAL(debug) << "receive " << bytes_transferred
                           << " bytes: " << line;
  this->buffer += line;

  for (;;) {
    const auto idx = this->match(this->buffer);
    if (!idx) {
      break;
    }
    if (idx->first > 0) {
      const std::string absent = this->buffer.substr(0, idx->first);
      BOOST_LOG_TRIVIAL(warning) << "absent message " << absent;
    }
    const std::string message =
        this->buffer.substr(idx->first, (idx->second - idx->first));
    BOOST_LOG_TRIVIAL(info) << "match message " << message;
    {
      this->buffer = this->buffer.substr(idx->second);
      BOOST_LOG_TRIVIAL(debug) << "current buffer " << this->buffer;
    }
    this->on_receive(message);
  }

  this->async_read();
}

std::optional<std::pair<size_t, size_t>> palm::SerialPort::match(
    const std::string& buffer, const std::string& header,
    const std::string& footer) const {
  const auto begin = buffer.find(header);
  const auto end = buffer.find(footer);
  if (begin != std::string::npos && end != std::string::npos) {
    return std::make_pair(begin, end + footer.size());
  }
  return std::nullopt;
}
