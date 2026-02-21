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
      .default_value(8080)
      .scan<'i', int>()
      .required();
  http_command.add_argument("-t", "--theme")
      .help("theme folder(bootstrap,bulma)")
      .default_value("views/bootstrap")
      .required();
  program.add_subparser(http_command);

  argparse::ArgumentParser db_seeds_command("db-seeds");
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
  if (program.is_subcommand_used(http_command)) {
    const int port = http_command.get<int>("--port");
    const std::string theme = http_command.get<std::string>("--theme");
    return this->http(config_file, port, theme);
  }
  if (program.is_subcommand_used(db_seeds_command)) {
    return this->db_seeds(config_file);
  }

  std::cout << program.help().str() << std::endl;
  return EXIT_SUCCESS;
}
