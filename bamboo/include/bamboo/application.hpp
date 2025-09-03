#pragma once

#include "palm/crypto.hpp"
#include "palm/jwt.hpp"
#include "palm/orm.hpp"

namespace bamboo {
class Application {
 public:
  Application() {}
  void launch(int argc, char* argv[]);

 private:
  void rpc_server(const toml::table& config, const std::string& host,
                  uint16_t port);
  void reboot();
  void apply(const std::string& input, bool run = false);
  void sample(const std::string& output);
  std::shared_ptr<soci::session> db(const toml::table& config);
  std::shared_ptr<palm::Jwt> jwt(const toml::table& config);
  std::shared_ptr<palm::Aes> aes(const toml::table& config);
  std::shared_ptr<palm::HMac> hmac(const toml::table& config);
};

}  // namespace bamboo
