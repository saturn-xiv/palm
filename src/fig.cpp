#include "palm/fig.hpp"
#include "palm/auth.hpp"
#include "palm/version.hpp"

#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>

static inline std::string parse_rpc_address() {
  std::stringstream ss;
  // TODO
  ss << "0.0.0.0:" << 8888;
  return ss.str();
}

void palm::fig::Application::rpc() {
  const auto address = parse_rpc_address();
  palm::auth::UserService auth_user;

  grpc::EnableDefaultHealthCheckService(true);
  grpc::reflection::InitProtoReflectionServerBuilderPlugin();
  grpc::ServerBuilder builder;

  builder.AddListeningPort(address, grpc::InsecureServerCredentials());
  builder.RegisterService(&auth_user);

  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
  BOOST_LOG_TRIVIAL(info) << "server listening on " << address;

  server->Wait();
}

void palm::fig::Application::web() {
  // TODO
}
void palm::fig::Application::worker() {
  // TODO
}
