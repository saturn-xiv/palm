#include "aloe/application.hpp"
#include "aloe/s3.hpp"
#include "palm/theme.hpp"
#include "palm/utils.hpp"
#include "palm/version.hpp"

#include <argparse/argparse.hpp>

void aloe::Application::launch(int argc, char* argv[]) {
  bool debug;
  std::string config_file;

  std::string s3_dump_host;

  std::string s3_restore_host;
  std::string s3_restore_file;

  std::string s3_sync_source;
  std::string s3_sync_destination;

  argparse::ArgumentParser program(
      "aloe", std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME));
  program.add_description("A collections of accessibility tools.");
  program.add_epilog(palm::PROJECT_HOME);
  program.add_argument("-c", "--config")
      .default_value("config.toml")
      .store_into(config_file)
      .help("configuration file");
  program.add_argument("-d", "--debug")
      .flag()
      .store_into(debug)
      .help("run on debug mode");

  argparse::ArgumentParser s3_dump_command("s3-dump");
  s3_dump_command.add_description("backup a s3 server");
  s3_dump_command.add_argument("-H", "--host")
      .required()
      .store_into(s3_dump_host);

  argparse::ArgumentParser s3_restore_command("s3-restore");
  s3_restore_command.add_description("restore a s3 server");
  s3_restore_command.add_argument("-H", "--host")
      .required()
      .store_into(s3_restore_host);
  s3_restore_command.add_argument("-f", "--file")
      .required()
      .store_into(s3_restore_file);

  argparse::ArgumentParser s3_sync_command("s3-sync");
  s3_sync_command.add_description("sync files between two s3 servers");
  s3_sync_command.add_argument("-s", "--source")
      .required()
      .store_into(s3_sync_source);
  s3_sync_command.add_argument("-d", "--destination")
      .required()
      .store_into(s3_sync_destination);
  s3_sync_command.add_argument("-l", "--file-list")
      .help("a (bucket,object) list in json format");

  program.add_subparser(s3_dump_command);
  program.add_subparser(s3_restore_command);
  program.add_subparser(s3_sync_command);
  program.parse_args(argc, argv);

  if (program.is_subcommand_used(s3_dump_command) ||
      program.is_subcommand_used(s3_restore_command) ||
      program.is_subcommand_used(s3_sync_command)) {
    palm::init(debug);
    spdlog::info("load configuration from {}", config_file);
    toml::table config = toml::parse_file(config_file);
    if (program.is_subcommand_used(s3_dump_command)) {
      aloe::s3::dump(config, s3_dump_host);
      return;
    }
    if (program.is_subcommand_used(s3_restore_command)) {
      aloe::s3::restore(config, s3_restore_host, s3_restore_file);
      return;
    }
    if (program.is_subcommand_used(s3_sync_command)) {
      auto file_list = s3_sync_command.present("-l");
      if (file_list) {
        aloe::s3::sync(config, s3_sync_source, s3_sync_destination,
                       file_list.value());
      } else {
        aloe::s3::sync(config, s3_sync_source, s3_sync_destination);
      }

      return;
    }
  }

  std::cout << program << std::endl;
}
