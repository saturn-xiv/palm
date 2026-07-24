#include "loquat/application.hpp"
#include "loquat/service.hpp"
#include "loquat/version.hpp"

#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>
#include <inja/inja.hpp>

void loquat::application::generate_systemd_config(const std::string& name,
                                                  const uint16_t port) {
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

static inline std::string _load_file(const std::string& path) {
  std::ifstream file(path);
  std::stringstream buf;
  buf << file.rdbuf();
  return buf.str();
}

void loquat::application::launch_rpc_server(
    const uint16_t port, std::optional<loquat::application::Ssl> ssl) {
  const std::string address = std::format("0.0.0.0:{}", port);
  loquat::JwtService jwt_service;
  loquat::HMacService hmac_service;
  loquat::AesService aes_service;
  loquat::Argon2Service argon2_service;

  grpc::EnableDefaultHealthCheckService(true);
  grpc::reflection::InitProtoReflectionServerBuilderPlugin();
  grpc::ServerBuilder builder;

  if (ssl) {
    spdlog::info("listening on tcps://0.0.0.0:{}", port);
    spdlog::debug("load cert from {}, key from {}, ca from {}", ssl->cert_file,
                  ssl->key_file, ssl->ca_file);
    const std::string server_key = _load_file(ssl->key_file);
    const std::string server_cert = _load_file(ssl->cert_file);
    const std::string root_ca = _load_file(ssl->ca_file);

    grpc::SslServerCredentialsOptions::PemKeyCertPair pkcp;
    pkcp.private_key = server_key;
    pkcp.cert_chain = server_cert;
    grpc::SslServerCredentialsOptions ssl_opts;
    ssl_opts.pem_key_cert_pairs.push_back(pkcp);
    ssl_opts.pem_root_certs = root_ca;
    ssl_opts.client_certificate_request =
        GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_AND_VERIFY;
    auto server_creds = grpc::SslServerCredentials(ssl_opts);
    builder.AddListeningPort(address, server_creds);
  } else {
    spdlog::info("listening on tcp://0.0.0.0:{}", port);
    builder.AddListeningPort(address, grpc::InsecureServerCredentials());
  }
  builder.RegisterService(&jwt_service);
  builder.RegisterService(&aes_service);
  builder.RegisterService(&hmac_service);
  builder.RegisterService(&argon2_service);
  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
  server->Wait();
}
