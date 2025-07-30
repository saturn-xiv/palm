#include "palm/services.hpp"

grpc::Status palm::monitoring::services::PodmanServiceImpl::Logs(
    grpc::ServerContext* context,
    const palm::monitoring::v1::PodmanQueryRequest* request,
    palm::monitoring::v1::PodmanLogsResponse* reply) {
  palm::CurrentUser current_user(context);
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
grpc::Status palm::monitoring::services::PodmanServiceImpl::Containers(
    grpc::ServerContext* context,
    const palm::monitoring::v1::PodmanQueryRequest* request,
    palm::monitoring::v1::PodmanContainersResponse* reply) {
  palm::CurrentUser current_user(context);
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
grpc::Status palm::monitoring::services::PodmanServiceImpl::Statistics(
    grpc::ServerContext* context,
    const palm::monitoring::v1::PodmanQueryRequest* request,
    palm::monitoring::v1::PodmanStatisticsResponse* reply) {
  palm::CurrentUser current_user(context);
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
