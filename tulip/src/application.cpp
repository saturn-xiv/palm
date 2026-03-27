#include "tulip/application.hpp"
#include "palm/version.hpp"
#include "tulip/portal.hpp"

#include <argparse/argparse.hpp>

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
      .default_value(static_cast<uint16_t>(8080))
      .scan<'i', uint16_t>()
      .required();
  http_command.add_argument("-t", "--threads")
      .help("run the I/O service on the requested number of threads")
      .default_value(std::thread::hardware_concurrency())
      .scan<'i', unsigned int>()
      .required();
  http_command.add_argument("-T", "--theme")
      .help("theme folder(bootstrap,bulma)")
      .default_value("views/bootstrap")
      .required();
  http_command.add_argument("-d", "--document-root")
      .help("document root folder")
      .default_value(std::filesystem::current_path().string())
      .required();
  program.add_subparser(http_command);

  argparse::ArgumentParser rpc_command("rpc");
  rpc_command.add_description("start a gRPC server");
  rpc_command.add_argument("-p", "--port")
      .help("listening on")
      .default_value(static_cast<uint16_t>(8080))
      .scan<'i', uint16_t>()
      .required();
  program.add_subparser(rpc_command);

  argparse::ArgumentParser db_seeds_command("db-seeds");
  db_seeds_command.add_description("load data from filesystem into db");
  program.add_subparser(db_seeds_command);

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
  if (!palm::config_file_permission(config_file)) {
    spdlog::error("invalid file permissions, must be 400 or 600");
    return EXIT_FAILURE;
  }
  if (program.is_subcommand_used(rpc_command)) {
    const uint16_t port = rpc_command.get<uint16_t>("--port");
    return this->rpc(config_file, port);
  }
  if (program.is_subcommand_used(http_command)) {
    const uint16_t port = http_command.get<uint16_t>("--port");
    const unsigned int threads = http_command.get<unsigned int>("--threads");
    const std::string theme = http_command.get<std::string>("--theme");
    const std::string document_root =
        http_command.get<std::string>("--document-root");
    if (threads < 2) {
      spdlog::error("threads must lager than 2");
      return EXIT_FAILURE;
    }
    if (!std::filesystem::exists(document_root) ||
        !std::filesystem::is_directory(document_root)) {
      spdlog::error("document root {} didn't exists", document_root);
      return EXIT_FAILURE;
    }
    if (!std::filesystem::exists(theme) ||
        !std::filesystem::is_directory(theme)) {
      spdlog::error("theme root {} didn't exists", theme);
      return EXIT_FAILURE;
    }

    return this->http(config_file, port, static_cast<size_t>(threads),
                      document_root, theme);
  }
  if (program.is_subcommand_used(db_seeds_command)) {
    return this->db_seeds(config_file);
  }

  std::cout << program.help().str() << std::endl;
  return EXIT_SUCCESS;
}
