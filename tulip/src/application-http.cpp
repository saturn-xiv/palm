
#include "tulip/application.hpp"
#include "tulip/http.hpp"
#include "tulip/portal.hpp"

#include <algorithm>
#include <csignal>
#include <cstdlib>
#include <functional>
#include <iostream>
#include <memory>
#include <thread>
#include <vector>

static std::function<void(int)> gl_shutdown_handler;

static void signal_handler(int signal) { gl_shutdown_handler(signal); }

int tulip::Application::http(const std::string& config_file, uint16_t port,
                             size_t threads,
                             const std::filesystem::path& document_root,
                             const std::filesystem::path& theme) const {
  struct Config {
    Config(const toml::table& config)
        : postgresql(*(config["postgresql"].as_table())),
          redis(*(config["redis"].as_table())),
          rabbitmq(*(config["rabbitmq"].as_table())),
          opensearch(*(config["opensearch"].as_table())) {}

    palm::PostgreSql postgresql;
    palm::redis::Config redis;
    palm::rabbitmq::Config rabbitmq;
    palm::opensearch::Config opensearch;
    palm::grpc::Config daisy;
  };

  if (palm::is_stopped()) {
    return EXIT_SUCCESS;
  }

  const auto config_tree = toml::parse_file(config_file);
  Config config(config_tree);

  tulip::portal::Context ctx;
  ctx.db = config.postgresql.open();
  ctx.cache = config.redis.open();
  ctx.queue = config.rabbitmq.open();
  {
    spdlog::debug("load theme from {}", theme.string());
    ctx.env = std::make_shared<inja::Environment>(theme);
  }
  {
    spdlog::debug("open opensearch {}", config.opensearch.url(""));
    ctx.search = std::make_shared<palm::opensearch::Config>(config.opensearch);

    {
      const auto info = ctx.search->info();
      spdlog::debug("node: {} {} v{}", info->name, info->cluster_uuid,
                    info->version.number);
    }
  }

  const std::map<std::string, std::filesystem::path> assets = {
      {"/statics", document_root / "assets"},
      {"/3rd", document_root / "node_modules"}};
  const std::string host = "0.0.0.0";
  spdlog::info("start to listening on http://{}:{}", host, port);
  spdlog::debug("with document root({}), theme({}), threads({})",
                document_root.string(), theme.string(), threads);

  tulip::http::boost_beast::Server server(threads);
  server.mount(ctx, assets);
  gl_shutdown_handler = [&](int signal) {
    if (signal == SIGINT) {
      spdlog::warn("Ctrl+C caught, exiting...");
      server.shutdown();
    }
  };
  std::signal(SIGINT, signal_handler);
  server.startup(host, port);
  return EXIT_SUCCESS;
}
