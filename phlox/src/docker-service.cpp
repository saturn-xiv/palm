#include "phlox/docker.hpp"
#include "phlox/services.hpp"

grpc::Status phlox::monitoring::services::DockerServiceImpl::Logs(
    grpc::ServerContext* context,
    const palm::monitoring::v1::PodmanQueryRequest* request,
    palm::monitoring::v1::PodmanLogsResponse* reply) {
  phlox::CurrentUser current_user(context);
  {
    auto name = current_user.name(this->_jwt);
    if (!name) {
      const grpc::Status it(grpc::StatusCode::UNAUTHENTICATED, "");
      return it;
    }
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status phlox::monitoring::services::DockerServiceImpl::Containers(
    grpc::ServerContext* context,
    const palm::monitoring::v1::PodmanQueryRequest* request,
    palm::monitoring::v1::DockerContainersResponse* reply) {
  phlox::CurrentUser current_user(context);
  {
    auto name = current_user.name(this->_jwt);
    if (!name) {
      const grpc::Status it(grpc::StatusCode::UNAUTHENTICATED, "");
      return it;
    }
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status phlox::monitoring::services::DockerServiceImpl::Statistics(
    grpc::ServerContext* context,
    const palm::monitoring::v1::PodmanQueryRequest* request,
    palm::monitoring::v1::DockerStatisticsResponse* reply) {
  phlox::CurrentUser current_user(context);
  {
    auto name = current_user.name(this->_jwt);
    if (!name) {
      const grpc::Status it(grpc::StatusCode::UNAUTHENTICATED, "");
      return it;
    }
  }
  // TODO
  return grpc::Status::OK;
}
