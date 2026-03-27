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

static std::function<void(int)> gl_shutdown_handler;

static void signal_handler(int signal) { gl_shutdown_handler(signal); }

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
  tulip::portal::rpc::service::Site portal_site_service;

  grpc::EnableDefaultHealthCheckService(true);
  // TODO
  // grpc::reflection::InitProtoReflectionServerBuilderPlugin();
  grpc::ServerBuilder builder;
  builder.AddListeningPort(address, grpc::InsecureServerCredentials());
  builder.RegisterService(&portal_site_service);
  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
  spdlog::info("listening on tcp://{}", address);
  server->Wait();

  return EXIT_SUCCESS;
}
