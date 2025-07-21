#include "palm/services.hpp"

grpc::Status palm::monitoring::services::PodmanServiceImpl::Logs(
    grpc::ServerContext* context,
    const palm::monitoring::v1::PodmanQueryRequest* request,
    palm::monitoring::v1::PodmanLogsResponse* reply) {
  // TODO
  return grpc::Status::OK;
}
grpc::Status palm::monitoring::services::PodmanServiceImpl::Containers(
    grpc::ServerContext* context,
    const palm::monitoring::v1::PodmanQueryRequest* request,
    palm::monitoring::v1::PodmanContainersResponse* reply) {
  // TODO
  return grpc::Status::OK;
}
grpc::Status palm::monitoring::services::PodmanServiceImpl::Statistics(
    grpc::ServerContext* context,
    const palm::monitoring::v1::PodmanQueryRequest* request,
    palm::monitoring::v1::PodmanStatisticsResponse* reply) {
  // TODO
  return grpc::Status::OK;
}
