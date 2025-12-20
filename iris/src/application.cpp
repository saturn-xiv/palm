#include "iris/application.hpp"
#include "iris/database.hpp"
#include "iris/filesystem.hpp"
#include "iris/minio.hpp"
#include "iris/utils.hpp"
#include "iris/version.hpp"

#include <algorithm>
#include <cstdlib>

#include <spdlog/spdlog.h>
#include <argparse/argparse.hpp>

int iris::Application::launch(int argc, char** argv) const {
  const std::string version =
      fmt::format("{}({})", iris::GIT_VERSION, iris::BUILD_TIME);
  argparse::ArgumentParser program(iris::PROJECT_NAME, version,
                                   argparse::default_arguments::help);

  program.add_description(iris::PROJECT_DESCRIPTION);
  program.add_epilog(iris::PROJECT_HOME);

  program.add_argument("-v", "--version").help("show version").flag();
  program.add_argument("-d", "--debug").help("run on debug mode").flag();

  argparse::ArgumentParser dump_command("dump");
  dump_command.add_description("Dump to file");
  dump_command.add_argument("-i", "--input")
      .help("input configuration file(toml)")
      .required();
  dump_command.add_argument("-o", "--output").help("output folder").required();
  dump_command.add_argument("-k", "--keep")
      .help("number of recent files to keep")
      .default_value(7)
      .scan<'i', int>()
      .required();
  dump_command.add_argument("-z", "--compress").help("compress").flag();
  program.add_subparser(dump_command);

  argparse::ArgumentParser restore_command("restore");
  restore_command.add_description("Restore from file");
  restore_command.add_argument("-i", "--input")
      .help("input file(xz)")
      .required();
  restore_command.add_argument("-o", "--output")
      .help("output configuration file(toml)")
      .required();
  program.add_subparser(restore_command);

  argparse::ArgumentParser generate_timer_command("generate-timer");
  generate_timer_command.add_description("Dump to file");
  generate_timer_command.add_argument("-n", "--name")
      .help("timer name(alphanumeric)")
      .required();
  program.add_subparser(generate_timer_command);

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

  {
    spdlog::set_level(program.get<bool>("--debug") == true
                          ? spdlog::level::debug
                          : spdlog::level::info);
    spdlog::debug("run on debug mode");
  }

  const std::string done = "done.";
  if (program.is_subcommand_used(dump_command)) {
    const std::string input = dump_command.get<std::string>("--input");
    const std::string output = dump_command.get<std::string>("--output");
    const int keep = dump_command.get<int>("--keep");
    const bool compress = dump_command.get<bool>("--compress");
    this->dump(input, output, compress, keep);
    spdlog::info(done);
    return EXIT_SUCCESS;
  }
  if (program.is_subcommand_used(restore_command)) {
    // TODO
    spdlog::info(done);
    return EXIT_SUCCESS;
  }
  if (program.is_subcommand_used(generate_timer_command)) {
    const std::string name = generate_timer_command.get<std::string>("--name");
    this->generate_timer(name);
    spdlog::info(done);
    return EXIT_SUCCESS;
  }

  std::cout << program.help().str() << std::endl;
  return EXIT_SUCCESS;
}
