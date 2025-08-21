#pragma once

#include "palm/orm.hpp"
#include "palm/search.hpp"

namespace chrysanthemum {
class Application {
 public:
  Application() {}
  void launch(int argc, char* argv[]);

 private:
  std::shared_ptr<soci::session> db(const toml::table& config);
  void generate_etc(const std::string& domain);
  void rpc_server(const toml::table& config, const std::string& host,
                  uint16_t port);
};
}  // namespace chrysanthemum
