#include "loquat/application.hpp"
#include "loquat/env.hpp"
#include "loquat/erlang.hpp"
#include "loquat/version.hpp"

#include <unistd.h>

#include <openssl/opensslv.h>
#include <tink/config/tink_config.h>
#include <tink/jwt/jwt_mac_config.h>
#include <tink/version.h>
#include <argparse/argparse.hpp>

void loquat::Application::launch(int argc, char** argv) const {
  const std::string version =
      loquat::GIT_VERSION + "(" + loquat::BUILD_TIME + ")";
  char hostname[HOST_NAME_MAX + 1];
  gethostname(hostname, sizeof(hostname));

  argparse::ArgumentParser program(loquat::PROJECT_NAME, version);
  program.add_description(loquat::PROJECT_DESCRIPTION);
  program.add_epilog("https://github.com/saturn-xiv/palm");
  program.add_argument("-d", "--debug")
      .default_value(false)
      .help("run on debug mode")
      .implicit_value(true);

  argparse::ArgumentParser node_command("c-node");
  {
    node_command.add_argument("-p", "--port")
        .default_value(9999)
        .required()
        .scan<'i', int>();
    node_command.add_argument("-n", "--nodename")
        .default_value(std::format("{}@{}", loquat::PROJECT_NAME, hostname))
        .required()
        .help("Erlang node name");
    node_command.add_argument("-c", "--cookie")
        .required()
        .help("a secret cookie string");
  }

  program.add_subparser(node_command);

  try {
    program.parse_args(argc, argv);
  } catch (const std::runtime_error& err) {
    spdlog::error("{}", err.what());
    std::exit(1);
  }

  {
    spdlog::set_level(program.get<bool>("--debug") ? spdlog::level::debug
                                                   : spdlog::level::info);
    spdlog::debug("run on debug mode {}", version);

    spdlog::debug("OpenSSL v{}", OPENSSL_VERSION_STR);
    spdlog::debug("Tink v{}", crypto::tink::Version::kTinkVersion);
    spdlog::debug(
        "Protocol Buffers v{}",
        google::protobuf::internal::VersionString(GOOGLE_PROTOBUF_VERSION));
  }
  {
    const auto status = crypto::tink::TinkConfig::Register();
    if (!status.ok()) {
      throw std::runtime_error("failed to register tink");
    }
  }
  {
    const auto status = crypto::tink::JwtMacRegister();
    if (!status.ok()) {
      throw std::runtime_error("failed to register tink-jwt");
    }
  }

  if (program.is_subcommand_used(node_command)) {
    const int port = node_command.get<int>("--port");
    const std::string cookie = node_command.get<std::string>("--cookie");
    const std::string nodename = node_command.get<std::string>("--nodename");

    loquat::erlang::CNode node(nodename, cookie, static_cast<uint16_t>(port));
    node.run();
  }
}
