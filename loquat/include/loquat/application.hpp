#pragma once

#include <cstdint>
#include <optional>
#include <string>

namespace loquat {
namespace application {
struct Ssl {
  Ssl(const std::string& cert_file, const std::string& key_file,
      const std::string& ca_file)
      : cert_file(cert_file), key_file(key_file), ca_file(ca_file) {}
  std::string cert_file;
  std::string key_file;
  std::string ca_file;
};

void launch_rpc_server(const uint16_t port, std::optional<Ssl> ssl);
void generate_systemd_config(const std::string& name, const uint16_t port);
}  // namespace application
}  // namespace loquat
