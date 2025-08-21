#include "chrysanthemum/application.hpp"
#include "chrysanthemum/babel.hpp"
#include "chrysanthemum/bbs.hpp"
#include "chrysanthemum/blog.hpp"
#include "chrysanthemum/chat.hpp"
#include "chrysanthemum/cms.hpp"
#include "chrysanthemum/ledger.hpp"
#include "chrysanthemum/logistics.hpp"
#include "chrysanthemum/mall.hpp"
#include "chrysanthemum/survey.hpp"
#include "palm/utils.hpp"
#include "palm/version.hpp"

#include <argparse/argparse.hpp>

void chrysanthemum::Application::launch(int argc, char* argv[]) {
  bool debug;
  std::string config_file;

  std::string rpc_listen_host;
  int rpc_listen_port;

  std::string generate_etc_domain;

  argparse::ArgumentParser program(
      "chrysanthemum",
      std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME));
  program.add_description("A collection of gRPC services.");
  program.add_epilog(palm::PROJECT_HOME);
  program.add_argument("-c", "--config")
      .default_value("config.toml")
      .store_into(config_file)
      .help("configuration file");
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

  argparse::ArgumentParser generate_etc_command("generate-etc");
  generate_etc_command.add_description("generate system configuration files");
  generate_etc_command.add_argument("-n", "--domain-name")
      .required()
      .store_into(generate_etc_domain);

  program.add_subparser(rpc_command);
  program.add_subparser(generate_etc_command);

  program.parse_args(argc, argv);

  if (program.is_subcommand_used(generate_etc_command)) {
    this->generate_etc(generate_etc_domain);
    return;
  }

  if (program.is_subcommand_used(rpc_command)) {
    palm::init(debug);
    spdlog::info("load configuration from {}", config_file);
    toml::table config = toml::parse_file(config_file);
    if (program.is_subcommand_used(rpc_command)) {
      this->rpc_server(config, rpc_listen_host, rpc_listen_port);
      return;
    }
  }
  std::cout << program << std::endl;
}

void chrysanthemum::Application::generate_etc(const std::string& domain) {
  // TODO
}
void chrysanthemum::Application::rpc_server(const toml::table& config,
                                            const std::string& host,
                                            uint16_t port) {
  // TODO
}

std::shared_ptr<soci::session> chrysanthemum::Application::db(
    const toml::table& config){
        // TODO
    }
