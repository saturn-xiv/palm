#include "bamboo/application.hpp"
#include "bamboo/services.hpp"
#include "palm/utils.hpp"

#include <grpcpp/health_check_service_interface.h>
#include <grpcpp/security/server_credentials.h>

void bamboo::Application::rpc_server(const toml::table& config,
                                     const std::string& host, uint16_t port) {
  if (palm::is_stopped()) {
    return;
  }

  const std::string address = std::format("{}:{}", host, port);

  auto db = this->db(config);
  auto jwt = this->jwt(config);
  auto aes = this->aes(config);

  {
    soci::transaction tr(*db);
    const std::string installed_at = "site.installed-at";
    const auto it = bamboo::dao::get(*db, installed_at);
    if (!it) {
      spdlog::warn("empty database, will be setup it at first");
      bamboo::dao::administrator::save(*db, "admin", "123456");
      {
        const auto now = google::protobuf::util::TimeUtil::GetCurrentTime();
        bamboo::dao::set(*db, installed_at, now);
      }
    }
    tr.commit();
  }

  bamboo::services::AdministratorServiceImpl administrator_service(db, jwt);
  bamboo::services::RouterServiceImpl router_service(db, jwt);
  bamboo::services::UserServiceImpl user_service(db, aes, jwt);
  bamboo::services::HostServiceImpl host_service(db, jwt);

  grpc::EnableDefaultHealthCheckService(true);

  grpc::ServerBuilder builder;
  builder.AddListeningPort(address, grpc::InsecureServerCredentials());

  {
    builder.RegisterService(&administrator_service);
    builder.RegisterService(&router_service);
    builder.RegisterService(&user_service);
    builder.RegisterService(&host_service);
  }

  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
  spdlog::info("listen a gRPC server on tcp://{}:{}", host, port);

  server->Wait();
}
