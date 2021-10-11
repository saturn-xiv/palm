#pragma once

#include "palm/env.hpp"

#include <libserial/SerialPort.h>

namespace palm {

class Tty {
 public:
  Tty(const std::filesystem::path& file);
  void write(const std::vector<uint8_t>& buf);
  void write(const std::string& buf);
  std::string read(
      const std::chrono::milliseconds& timeout = std::chrono::milliseconds(10));

 private:
  std::mutex mutex;
  LibSerial::SerialPort port;
};
}  // namespace palm
