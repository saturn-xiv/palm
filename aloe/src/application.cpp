#include "aloe/application.hpp"
#include "aloe/dm8.hpp"
#include "aloe/ftp.hpp"
#include "aloe/mysql.hpp"
#include "aloe/oracle.hpp"
#include "aloe/postgresql.hpp"
#include "aloe/s3.hpp"
#include "palm/theme.hpp"
#include "palm/utils.hpp"
#include "palm/version.hpp"

#include <argparse/argparse.hpp>

void aloe::Application::launch(int argc, char* argv[]) {
  bool debug;

  std::vector<std::string> s3_dump_hosts;
  bool s3_dump_zip;

  std::string s3_restore_host;
  std::string s3_restore_file;

  std::string s3_sync_source;
  std::string s3_sync_destination;

  std::string dm8_restore_host;
  std::string dm8_restore_directory;
  std::string dm8_restore_file;

  std::string dm8_dump_host;
  std::string dm8_dump_directory;
  bool dm8_dump_zip;

  const std::string work_dir = std::filesystem::current_path().string();

  argparse::ArgumentParser program(
      "aloe", std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME));
  program.add_description("A collections of accessibility tools.");
  program.add_epilog(palm::PROJECT_HOME);
  program.add_argument("-d", "--debug")
      .flag()
      .store_into(debug)
      .help("run on debug mode");

  argparse::ArgumentParser s3_dump_command("s3-dump");
  s3_dump_command.add_description("backup a s3 server");
  s3_dump_command.add_argument("-H", "--hosts")
      .required()
      .append()
      .store_into(s3_dump_hosts);
  s3_dump_command.add_argument("-z", "--zip")
      .flag()
      .store_into(s3_dump_zip)
      .help("compress the package(tar.xz)");

  argparse::ArgumentParser s3_restore_command("s3-restore");
  s3_restore_command.add_description("restore a s3 server");
  s3_restore_command.add_argument("-H", "--host")
      .required()
      .store_into(s3_restore_host);
  s3_restore_command.add_argument("-f", "--file")
      .required()
      .store_into(s3_restore_file);
  s3_restore_command.add_argument("-l", "--file-list")
      .help("a (bucket,object) list in json format");

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

  argparse::ArgumentParser dm8_dump_command("dm8-dump");
  dm8_dump_command.add_description("backup from DM8 database");
  dm8_dump_command.add_argument("-H", "--hostname")
      .required()
      .store_into(dm8_dump_host);
  dm8_dump_command.add_argument("-d", "--directory")
      .required()
      .default_value(work_dir)
      .store_into(dm8_dump_directory);
  dm8_dump_command.add_argument("-z", "--zip")
      .flag()
      .store_into(dm8_dump_zip)
      .help("compress the package(tar.xz)");

  argparse::ArgumentParser dm8_restore_command("dm8-restore");
  dm8_restore_command.add_description("restore to DM8 database ");
  dm8_restore_command.add_argument("-H", "--hostname")
      .required()
      .store_into(dm8_restore_host);
  dm8_restore_command.add_argument("-d", "--directory")
      .required()
      .default_value(work_dir)
      .store_into(dm8_restore_directory);
  dm8_restore_command.add_argument("-f", "--file")
      .required()
      .store_into(dm8_restore_file);

  program.add_subparser(s3_dump_command);
  program.add_subparser(s3_restore_command);
  program.add_subparser(s3_sync_command);
  program.add_subparser(dm8_dump_command);
  program.add_subparser(dm8_restore_command);
  program.parse_args(argc, argv);

  if (program.is_subcommand_used(s3_dump_command) ||
      program.is_subcommand_used(s3_restore_command) ||
      program.is_subcommand_used(s3_sync_command) ||
      program.is_subcommand_used(dm8_dump_command) ||
      program.is_subcommand_used(dm8_restore_command)) {
    palm::init(debug);

    if (program.is_subcommand_used(s3_dump_command)) {
      std::set<std::string> hosts(s3_dump_hosts.begin(), s3_dump_hosts.end());
      aloe::s3::dump(hosts, s3_dump_zip);
      return;
    }
    if (program.is_subcommand_used(s3_restore_command)) {
      auto file_list = s3_restore_command.present("-l");
      if (file_list) {
        aloe::s3::restore(s3_restore_host, s3_restore_file, file_list.value());
      } else {
        aloe::s3::restore(s3_restore_host, s3_restore_file);
      }

      return;
    }
    if (program.is_subcommand_used(s3_sync_command)) {
      auto file_list = s3_sync_command.present("-l");
      if (file_list) {
        aloe::s3::sync(s3_sync_source, s3_sync_destination, file_list.value());
      } else {
        aloe::s3::sync(s3_sync_source, s3_sync_destination);
      }
      return;
    }
    if (program.is_subcommand_used(dm8_dump_command)) {
      const std::string config_file = std::format("{}.toml", dm8_dump_host);
      spdlog::debug("load configuration from {}", config_file);
      const toml::table config = toml::parse_file(config_file);
      aloe::Dm8 dm8(config);
      dm8.dump(dm8_dump_directory, dm8_dump_zip);
      return;
    }
    if (program.is_subcommand_used(dm8_restore_command)) {
      const std::string config_file = std::format("{}.toml", dm8_restore_host);
      spdlog::debug("load configuration from {}", config_file);
      const toml::table config = toml::parse_file(config_file);
      aloe::Dm8 dm8(config);
      dm8.restore(dm8_restore_directory, dm8_restore_file);
      return;
    }
  }

  std::cout << program << std::endl;
}
