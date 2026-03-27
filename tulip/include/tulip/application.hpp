#pragma once

#include <cstdint>
#include <filesystem>
#include <string>

namespace tulip {
class Application {
 public:
  Application() {}
  int launch(int argc, char** argv) const;

 private:
  int http(const std::string& config_file, uint16_t port, size_t threads,
           const std::filesystem::path& document_root,
           const std::filesystem::path& theme) const;
  int rpc(const std::string& config_file, uint16_t port) const;
  int db_seeds(const std::string& config_file) const;
};
}  // namespace tulip
