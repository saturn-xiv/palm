#include "bamboo/application.hpp"
#include "palm/utils.hpp"
#include "palm/version.hpp"

#include <argparse/argparse.hpp>

void bamboo::Application::launch(int argc, char* argv[]) {
  bool debug;

  std::string rpc_config_file;
  std::string rpc_listen_host;
  int rpc_listen_port;

  std::string apply_input_file;
  bool apply_run;

  std::string sample_output_file;

  std::string scan_config_file;

  argparse::ArgumentParser program(
      "bamboo", std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME));
  program.add_description("A smart router inspired by OpenWrt.");
  program.add_epilog(palm::PROJECT_HOME);
  program.add_argument("-d", "--debug")
      .flag()
      .store_into(debug)
      .help("run on debug mode");

  argparse::ArgumentParser rpc_command("rpc");
  rpc_command.add_description("start a gRPC server");
  rpc_command.add_argument("-H", "--host")
      .default_value("127.0.0.1")
      .store_into(rpc_listen_host);
  rpc_command.add_argument("-p", "--port")
      .default_value(8080)
      .store_into(rpc_listen_port);
  rpc_command.add_argument("-c", "--config")
      .default_value("config.toml")
      .store_into(rpc_config_file)
      .help("configuration file");

  argparse::ArgumentParser reboot_command("reboot");
  reboot_command.add_description("reboot the system");

  argparse::ArgumentParser scan_command("scan");
  scan_command.add_description("scan the internal hosts");
  scan_command.add_argument("-c", "--config")
      .default_value("config.toml")
      .store_into(scan_config_file)
      .help("configuration file");

  argparse::ArgumentParser apply_command("apply");
  apply_command.add_description("apply from configuration");
  apply_command.add_argument("-i", "--input")
      .default_value("config.json")
      .store_into(apply_input_file)
      .help("configuration file");
  apply_command.add_argument("-r", "--run")
      .flag()
      .store_into(apply_run)
      .help("run it after generate the script file");

  argparse::ArgumentParser sample_command("sample");
  sample_command.add_description("generate a sample resource file");
  sample_command.add_argument("-o", "--output")
      .default_value("config.json")
      .store_into(sample_output_file)
      .help("configuration file");

  program.add_subparser(rpc_command);
  program.add_subparser(scan_command);
  program.add_subparser(reboot_command);
  program.add_subparser(sample_command);
  program.add_subparser(apply_command);
  program.parse_args(argc, argv);

  if (program.is_subcommand_used(reboot_command)) {
    palm::init(debug);
    this->reboot();
    return;
  }

  if (program.is_subcommand_used(apply_command)) {
    palm::init(debug);
    this->apply(apply_input_file, apply_run);
    return;
  }
  if (program.is_subcommand_used(sample_command)) {
    palm::init(debug);
    this->sample(sample_output_file);
    return;
  }

  if (program.is_subcommand_used(rpc_command)) {
    palm::init(debug);
    spdlog::info("load configuration from {}", rpc_config_file);
    toml::table config = toml::parse_file(rpc_config_file);
    if (program.is_subcommand_used(rpc_command)) {
      this->rpc_server(config, rpc_listen_host, rpc_listen_port);
      return;
    }
  }
  if (program.is_subcommand_used(scan_command)) {
    palm::init(debug);
    spdlog::info("load configuration from {}", scan_config_file);
    toml::table config = toml::parse_file(scan_config_file);
    this->scan(config);
    return;
  }
  std::cout << program << std::endl;
}
