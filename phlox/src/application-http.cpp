#include "phlox/application.hpp"
#include "phlox/controllers.hpp"

static inline std::shared_ptr<grpc::Channel> open_backend(
    const toml::table& config) {
  auto node = config["backend"].as_table();
  if (node == nullptr) {
    spdlog::error("missing backend part");
    return nullptr;
  }
  palm::GRpcClient cfg(*node);
  return cfg.open();
}

void phlox::Application::http_server(const toml::table& config,
                                     const std::string& host, uint16_t port) {
  if (palm::is_stopped()) {
    return;
  }

  auto jwt = this->jwt(config);
  auto channel = open_backend(config);

  httplib::Server server;
  palm::set_logger(server);
  phlox::mount(server, jwt, channel);

  spdlog::info("listen a HTTP server on tcp://{}:{}", host, port);
  server.listen(host, port);
}
