#include "tulip/accounting.hpp"
#include "tulip/application.hpp"
#include "tulip/babel.hpp"
#include "tulip/blog.hpp"
#include "tulip/cms.hpp"
#include "tulip/forum.hpp"
#include "tulip/portal.hpp"

// #include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>

static grpc::Server* gl_running_server = nullptr;
static void signal_handler(int signal) {
  if (gl_running_server == nullptr) {
    return;
  }
  switch (signal) {
    case SIGINT:
      spdlog::warn("Ctrl+C caught, exiting...");
      break;
    case SIGTERM:
      spdlog::warn("Terminated caught, exiting...");
      break;
  }
  std::thread it([] { gl_running_server->Shutdown(); });
  it.join();
}

int tulip::Application::rpc(const std::string& config_file,
                            uint16_t port) const {
  struct Config {
    Config(const toml::table& config)
        : postgresql(*(config["postgresql"].as_table())),
          redis(*(config["redis"].as_table())),
          rabbitmq(*(config["rabbitmq"].as_table())),
          opensearch(*(config["opensearch"].as_table())),
          daisy(*(config["daisy"].as_table())) {}

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

  const std::string address = std::format("0.0.0.0:{}", port);

  grpc::EnableDefaultHealthCheckService(true);
  // TODO grpc codegen feature
  // grpc::reflection::InitProtoReflectionServerBuilderPlugin();
  grpc::ServerBuilder builder;
  builder.AddListeningPort(address, grpc::InsecureServerCredentials());

  tulip::portal::rpc::service::Site portal_site_service;
  builder.RegisterService(&portal_site_service);

  spdlog::info("listening on tcp://{}", address);
  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());

  gl_running_server = server.get();
  std::signal(SIGINT, signal_handler);
  std::signal(SIGTERM, signal_handler);

  server->Wait();
  return EXIT_SUCCESS;
}
