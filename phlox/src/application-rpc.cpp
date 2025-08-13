#include "phlox/application.hpp"
#include "phlox/services.hpp"

#include <grpcpp/health_check_service_interface.h>
#include <grpcpp/security/server_credentials.h>

void phlox::Application::rpc_server(const toml::table& config,
                                    const std::string& host, uint16_t port) {
  if (palm::is_stopped()) {
    return;
  }

  const std::string address = std::format("{}:{}", host, port);

  auto search = this->opensearch(config);
  auto jwt = this->jwt(config);

  phlox::monitoring::services::SiteServiceImpl site_service(jwt, search);
  phlox::monitoring::services::PodmanServiceImpl podman_service(jwt, search);
  phlox::monitoring::services::SystemdServiceImpl systemd_service(jwt, search);
  phlox::monitoring::services::FileSystemServiceImpl file_system_service(
      jwt, search);

  grpc::EnableDefaultHealthCheckService(true);
  // TODO
  // grpc::reflection::InitProtoReflectionServerBuilderPlugin();

  grpc::ServerBuilder builder;
  builder.AddListeningPort(address, grpc::InsecureServerCredentials());

  {
    builder.RegisterService(&site_service);
    builder.RegisterService(&podman_service);
    builder.RegisterService(&systemd_service);
    builder.RegisterService(&file_system_service);
  }

  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
  spdlog::info("listen a gRPC server on tcp://{}:{}", host, port);

  server->Wait();
}
