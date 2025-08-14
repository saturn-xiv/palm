#pragma once

#include "palm/jwt.hpp"
#include "palm/orm.hpp"
#include "palm/rpc.hpp"
#include "palm/search.hpp"

namespace phlox {
class Application {
 public:
  Application() {}
  void launch(int argc, char* argv[]);

 private:
  std::shared_ptr<soci::session> db(const toml::table& config);
  std::shared_ptr<palm::Jwt> jwt(const toml::table& config);
  std::shared_ptr<palm::opensearch::Client> opensearch(
      const toml::table& config);

  void podman_logs(const toml::table& config);
  void podman_stats(const toml::table& config, bool all);
  void podman_ps(const toml::table& config, bool all);
  void docker_logs(const toml::table& config);
  void docker_stats(const toml::table& config, bool all);
  void docker_ps(const toml::table& config, bool all);
  void systemd_journal(const toml::table& config,
                       const std::string& service_name, bool user_scope);
  void generate_etc(const std::string& domain);
  void generate_token(const toml::table& config, const std::string& username,
                      uint8_t years);
  void fs_watcher(const toml::table& config, bool stdin,
                  const std::set<std::string>& original_files);
  void http_server(const toml::table& config, const std::string& host,
                   uint16_t port);
  void rpc_server(const toml::table& config, const std::string& host,
                  uint16_t port);
};
}  // namespace phlox
