#include "loquat/application.hpp"
#include "loquat/service.hpp"

#include <csignal>

// #include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>

std::function<void(int)> gl_signal_handler;

static void signal_router(int signum) {
  if (gl_signal_handler) {
    gl_signal_handler(signum);
  }
}

static inline std::string _load_file(const std::string& path) {
  std::ifstream file(path);
  std::stringstream buf;
  buf << file.rdbuf();
  return buf.str();
}

void loquat::Application::launch_rpc_server(
    uint16_t port, std::optional<loquat::Ssl> ssl) const {
  const std::string address = std::format("0.0.0.0:{}", port);
  loquat::JwtService jwt_service;
  loquat::HMacService hmac_service;
  loquat::AesService aes_service;
  loquat::Argon2Service argon2_service;

  grpc::EnableDefaultHealthCheckService(true);
  // TODO
  // grpc::reflection::InitProtoReflectionServerBuilderPlugin();
  grpc::ServerBuilder builder;

  if (ssl) {
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
    builder.AddListeningPort(address, grpc::InsecureServerCredentials());
  }
  builder.RegisterService(&jwt_service);
  builder.RegisterService(&aes_service);
  builder.RegisterService(&hmac_service);
  builder.RegisterService(&argon2_service);
  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());

  {
    grpc::HealthCheckServiceInterface* health_service =
        server->GetHealthCheckService();
    if (health_service) {
      health_service->SetServingStatus(true);
    }
  }

  {
    grpc::Server* running_server = server.get();
    gl_signal_handler = [running_server](int signal) {
      if (running_server == nullptr) {
        return;
      }
      switch (signal) {
        case SIGINT:
          spdlog::warn("Ctrl+C caught, exiting...");
          break;
        case SIGTERM:
          spdlog::warn("terminated caught, exiting...");
          break;
        default:
          const std::string err = std::format("ignore signal {}", signal);
          spdlog::error(err);
          return;
      }
      std::thread it([running_server] { running_server->Shutdown(); });
      it.join();
    };
    std::signal(SIGINT, signal_router);
    std::signal(SIGTERM, signal_router);
  }

  spdlog::info("listening on tcp://0.0.0.0:{}", port);
  server->Wait();
}
