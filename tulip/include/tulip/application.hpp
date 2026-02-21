#pragma once

#include <cstdint>
#include <string>

namespace tulip {
class Application {
 public:
  Application() {}
  int launch(int argc, char** argv) const;

 private:
  int http(const std::string& config_file, uint16_t port,
           const std::string& theme) const;
  int db_seeds(const std::string& config_file) const;
};
}  // namespace tulip
