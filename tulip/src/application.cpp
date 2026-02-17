#include "tulip/application.hpp"
#include "palm/version.hpp"
#include "tulip/accounting.hpp"
#include "tulip/babel.hpp"
#include "tulip/blog.hpp"
#include "tulip/cms.hpp"
#include "tulip/forum.hpp"

#include <csignal>
#include <functional>

#include <argparse/argparse.hpp>

static std::function<void(int)> shutdown_handler;

static void signal_handler(int signal) { shutdown_handler(signal); }

int tulip::Application::http(const std::string& config_file,
                             uint16_t port) const {
  struct Config {
    Config(const toml::table& config)
        : postgresql(*(config["postgresql"].as_table())),
          redis(*(config["redis"].as_table())),
          rabbitmq(*(config["rabbitmq"].as_table())) {}

    palm::PostgreSql postgresql;
    palm::redis::Config redis;
    palm::rabbitmq::Config rabbitmq;
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

  const std::string host = "0.0.0.0";
  spdlog::info("start to listening on http://{}:{}", host, port);
  httplib::Server server;

  shutdown_handler = [&](int signal) {
    if (signal == SIGINT) {
      spdlog::warn("Ctrl+C caught, exiting...");
      server.stop();
    }
  };
  std::signal(SIGINT, signal_handler);
  server.listen(host, port);
  return EXIT_SUCCESS;
}

int tulip::Application::launch(int argc, char** argv) const {
  const std::string version =
      fmt::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME);
  argparse::ArgumentParser program(palm::PROJECT_NAME, version,
                                   argparse::default_arguments::help);

  program.add_description(palm::PROJECT_DESCRIPTION);
  program.add_epilog(palm::PROJECT_HOME);

  program.add_argument("-v", "--version").help("show version").flag();
  program.add_argument("-d", "--debug").help("run on debug mode").flag();
  program.add_argument("-c", "--config")
      .help("load configuration from")
      .default_value("config.toml")
      .required();

  argparse::ArgumentParser http_command("http");
  http_command.add_description("start a HTTP server");
  http_command.add_argument("-p", "--port")
      .help("listening on")
      .default_value(8080)
      .scan<'i', int>()
      .required();

  program.add_subparser(http_command);

  try {
    program.parse_args(argc, argv);
  } catch (const std::exception& err) {
    spdlog::error("{}", err.what());
    return EXIT_FAILURE;
  }

  if (program.get<bool>("--version") == true) {
    std::cout << version << std::endl;
    return EXIT_SUCCESS;
  }

  const std::string config_file = program.get<std::string>("--config");
  palm::init(program.get<bool>("--debug"));
  spdlog::debug("load configuration from {}", config_file);
  if (program.is_subcommand_used(http_command)) {
    const int port = http_command.get<int>("--port");
    return this->http(config_file, port);
  }

  std::cout << program.help().str() << std::endl;
  return EXIT_SUCCESS;
}
