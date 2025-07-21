#include "palm/services.hpp"

grpc::Status palm::monitoring::services::FileSystemServiceImpl::Logs(
    grpc::ServerContext* context,
    const palm::monitoring::v1::FileSystemLogsRequest* request,
    palm::monitoring::v1::FileSystemLogsResponse* reply) {
  // TODO
  return grpc::Status::OK;
}
