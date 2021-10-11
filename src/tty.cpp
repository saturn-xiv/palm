#include "palm/tty.hpp"

palm::Tty::Tty(const std::filesystem::path& file) {
  BOOST_LOG_TRIVIAL(info) << "open serial port " << file;
  this->port.Open(file.string());
  this->port.SetBaudRate(LibSerial::BaudRate::BAUD_9600);
  this->port.SetCharacterSize(LibSerial::CharacterSize::CHAR_SIZE_8);
  this->port.SetFlowControl(LibSerial::FlowControl::FLOW_CONTROL_NONE);
  this->port.SetParity(LibSerial::Parity::PARITY_NONE);
  this->port.SetStopBits(LibSerial::StopBits::STOP_BITS_1);
}

void palm::Tty::write(const std::string& buf) {
  BOOST_LOG_TRIVIAL(info) << "write to serial port " << buf;
  std::lock_guard<std::mutex> lock(this->mutex);
  this->port.Write(buf);
  this->port.DrainWriteBuffer();
}

void palm::Tty::write(const std::vector<uint8_t>& buf) {
  BOOST_LOG_TRIVIAL(info) << "write " << buf.size() << " bytes to serial port";
  std::lock_guard<std::mutex> lock(this->mutex);
  for (auto const it : buf) {
    this->port.WriteByte(it);
  }
  this->port.DrainWriteBuffer();
}

std::string palm::Tty::read(const std::chrono::milliseconds& timeout) {
  std::lock_guard<std::mutex> lock(this->mutex);
  std::stringstream line;
  try {
    char it;
    this->port.ReadByte(it, timeout.count());
    line << it;
  } catch (const LibSerial::ReadTimeout&) {
  }
  return line.str();
}
