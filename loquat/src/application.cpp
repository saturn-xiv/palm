#include "loquat/application.hpp"
#include "loquat/env.hpp"
#include "loquat/version.hpp"

#include <grpcpp/grpcpp.h>
#include <openssl/opensslv.h>
#include <sodium.h>
#include <tink/config/tink_config.h>
#include <tink/jwt/jwt_mac_config.h>
#include <tink/version.h>
#include <argparse/argparse.hpp>
#include <inja/inja.hpp>

int loquat::Application::launch(int argc, char** argv) const {
  const std::string version =
      loquat::GIT_VERSION + "(" + loquat::BUILD_TIME + ")";
  argparse::ArgumentParser program(loquat::PROJECT_NAME, version);
  program.add_description(loquat::PROJECT_DESCRIPTION);
  program.add_epilog("https://github.com/saturn-xiv/palm/tree/main/loquat");
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
    return EXIT_FAILURE;
  }

  {
    const bool debug = program.get<bool>("--debug");
    if (!this->init(debug, version)) {
      return EXIT_FAILURE;
    }
  }

  if (program.is_subcommand_used(rpc_command)) {
    const int port = rpc_command.get<int>("--port");
    const std::string cert_file = rpc_command.get<std::string>("--cert-file");
    const std::string key_file = rpc_command.get<std::string>("--key-file");
    const std::string ca_file = rpc_command.get<std::string>("--ca-file");
    const auto ssl =
        std::make_optional<loquat::Ssl>(cert_file, key_file, ca_file);

    this->launch_rpc_server(
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
    this->generate_token(key_id, issuer, audience, subject,
                         static_cast<int16_t>(years));

  } else if (program.is_subcommand_used(systemd_config_command)) {
    const int port = systemd_config_command.get<int>("--port");
    const std::string name = systemd_config_command.get<std::string>("--name");
    this->generate_systemd_config(name, port);
  }

  return EXIT_SUCCESS;
}

bool loquat::Application::init(bool debug, const std::string& version) const {
  spdlog::set_level(debug ? spdlog::level::debug : spdlog::level::info);
  {
    spdlog::debug("run on debug mode {}", version);

    spdlog::debug("OpenSSL v{}", OPENSSL_VERSION_STR);
    spdlog::debug("Tink v{}", crypto::tink::Version::kTinkVersion);
    spdlog::debug("Libsodium v{}", SODIUM_VERSION_STRING);
    spdlog::debug(
        "Protocol Buffers v{}",
        google::protobuf::internal::VersionString(GOOGLE_PROTOBUF_VERSION));
    spdlog::debug("gRPC v{}", grpc::Version());
  }
  if (sodium_init() < 0) {
    spdlog::error(
        "the libsodium couldn't be initialized; it is not safe to use");
    return false;
  }
  {
    const auto status = crypto::tink::TinkConfig::Register();
    if (!status.ok()) {
      spdlog::error("failed to register tink");
      return false;
    }
  }
  {
    const auto status = crypto::tink::JwtMacRegister();
    if (!status.ok()) {
      spdlog::error("failed to register tink-jwt");
      return false;
    }
  }
  return true;
}

void loquat::Application::generate_systemd_config(const std::string& name,
                                                  uint16_t port) const {
  const std::filesystem::path file(name + ".conf");
  spdlog::info("generate file {}", file.string());
  nlohmann::json data = {
      {"name", name},
      {"description", PROJECT_DESCRIPTION},
      {"port", port},
  };
  std::ofstream output(file);
  inja::render_to(output, R"PLAIN(
[Unit]
Description={{ description }}
Wants=network-online.target
After=network-online.target

[Service]
Type=notify
User=root
Group=root
ExecStart=/usr/bin/loquat rpc -p {{ port }}
WorkingDirectory=/var/lib/{{ name }}
Restart=always

[Install]
WantedBy=multi-user.target
)PLAIN",
                  data);
}

void loquat::Application::generate_token(const std::string& key_id,
                                         const std::string& issuer,
                                         const std::string& audience,
                                         const std::string& subject,
                                         int16_t years) const {
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

  std::cout << token.value() << std::endl;
}
