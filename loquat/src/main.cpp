#include "loquat/service.hpp"
#include "loquat/version.hpp"

#include <event2/event.h>
#include <openssl/opensslv.h>
#include <tink/config/tink_config.h>
#include <tink/jwt/jwt_mac_config.h>
#include <tink/version.h>
#include <argparse/argparse.hpp>

namespace loquat {
static void set_thrift_logger(const char* s) { spdlog::debug("{}", s); }
}  // namespace loquat

int main(int argc, char** argv) {
  const std::string version =
      loquat::GIT_VERSION + "(" + loquat::BUILD_TIME + ")";
  argparse::ArgumentParser program(loquat::PROJECT_NAME, version);
  program.add_description(loquat::PROJECT_DESCRIPTION);
  program.add_epilog("https://github.com/saturn-xiv/palm");
  program.add_argument("-d", "--debug")
      .default_value(false)
      .help("run on debug mode")
      .implicit_value(true);

  argparse::ArgumentParser generate_token_command("generate-token");
  {
    generate_token_command.add_argument("-y", "--years")
        .default_value(10)
        .scan<'i', int>();
    generate_token_command.add_argument("-i", "--issuer").required();
    generate_token_command.add_argument("-s", "--subject").required();
    generate_token_command.add_argument("-a", "--audience").required();
    generate_token_command.add_argument("-k", "--key-id").required();
  }

  argparse::ArgumentParser rpc_command("rpc");
  {
    rpc_command.add_argument("-p", "--port")
        .default_value(9999)
        .scan<'i', int>();
    rpc_command.add_argument("-s", "--ssl")
        .default_value(false)
        .help("enable mutual tls mode")
        .implicit_value(true);
    rpc_command.add_argument("--cert-file")
        .default_value("server.crt")
        .required();
    rpc_command.add_argument("--key-file")
        .default_value("server.key")
        .required();
    rpc_command.add_argument("--ca-file").default_value("ca.crt").required();
  }

  argparse::ArgumentParser systemd_config_command("systemd");
  {
    systemd_config_command.add_argument("-p", "--port")
        .default_value(9999)
        .scan<'i', int>();
    systemd_config_command.add_argument("-n", "--name")
        .default_value(loquat::PROJECT_NAME)
        .required();
  }

  program.add_subparser(rpc_command);
  program.add_subparser(generate_token_command);
  program.add_subparser(systemd_config_command);

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
    spdlog::debug("libevent v{}", event_get_version());
    spdlog::debug("Tink v{}", crypto::tink::Version::kTinkVersion);
    spdlog::debug(
        "Protocol Buffers v{}",
        google::protobuf::internal::VersionString(GOOGLE_PROTOBUF_VERSION));
    spdlog::debug("Thrift v{}", loquat::thrift_version());
  }
  {
    const auto status = crypto::tink::TinkConfig::Register();
    if (!status.ok()) {
      spdlog::error("failed to register tink");
      return EXIT_FAILURE;
    }
  }
  {
    const auto status = crypto::tink::JwtMacRegister();
    if (!status.ok()) {
      spdlog::error("failed to register tink-jwt");
      return EXIT_FAILURE;
    }
  }

  if (program.is_subcommand_used(rpc_command)) {
    const int port = rpc_command.get<int>("--port");
    const std::string cert_file = rpc_command.get<std::string>("--cert-file");
    const std::string key_file = rpc_command.get<std::string>("--key-file");
    const std::string ca_file = rpc_command.get<std::string>("--ca-file");
    const auto ssl = std::make_optional<loquat::application::Ssl>(
        cert_file, key_file, ca_file);

    apache::thrift::GlobalOutput.setOutputFunction(loquat::set_thrift_logger);

    loquat::application::launch_rpc_server(
        static_cast<uint16_t>(port),
        rpc_command.get<bool>("--ssl") ? ssl : std::nullopt);

  } else if (program.is_subcommand_used(generate_token_command)) {
    const int years = generate_token_command.get<int>("--years");
    const std::string issuer =
        generate_token_command.get<std::string>("--issuer");
    const std::string key_id =
        generate_token_command.get<std::string>("--key-id");
    const std::string subject =
        generate_token_command.get<std::string>("--subject");
    const std::string audience =
        generate_token_command.get<std::string>("--audience");
    spdlog::warn("generate token to (kid: {}, aud: {}, sub: {}) for {}-years",
                 key_id, audience, subject, years);

    const auto ttl = std::chrono::duration_cast<std::chrono::seconds>(
        std::chrono::years(years));
    loquat::Jwt jwt;
    std::set<std::string> audiences{audience};

    auto now = absl::Now();
    const auto token =
        jwt.sign(std::nullopt, std::optional<std::string>{key_id}, issuer,
                 subject, audiences, now, now - absl::Seconds(1),
                 now + absl::Seconds(ttl.count()), std::nullopt);

    std::cout << token << std::endl;
  } else if (program.is_subcommand_used(systemd_config_command)) {
    const int port = systemd_config_command.get<int>("--port");
    const std::string name = systemd_config_command.get<std::string>("--name");
    loquat::application::generate_systemd_config(name, port);
  }

  return EXIT_SUCCESS;
}
