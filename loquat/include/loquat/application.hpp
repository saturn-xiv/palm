#pragma once

#include <cstdint>
#include <optional>
#include <string>

namespace loquat {
struct Ssl {
  Ssl(const std::string& cert_file, const std::string& key_file,
      const std::string& ca_file)
      : cert_file(cert_file), key_file(key_file), ca_file(ca_file) {}
  std::string cert_file;
  std::string key_file;
  std::string ca_file;
};
class Application {
 public:
  Application() {}
  int launch(int argc, char** argv) const;

 private:
  bool init(bool debug, const std::string& version) const;
  void launch_rpc_server(uint16_t port, std::optional<Ssl> ssl) const;
  void generate_systemd_config(const std::string& name, uint16_t port) const;
  void generate_token(const std::string& key_id, const std::string& issuer,
                      const std::string& audience, const std::string& subject,
                      int16_t years) const;
};

}  // namespace loquat
