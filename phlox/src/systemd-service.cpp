#include "phlox/services.hpp"

grpc::Status phlox::monitoring::services::SystemdServiceImpl::Journal(
    grpc::ServerContext* context,
    const palm::monitoring::v1::SystemdJournalRequest* request,
    palm::monitoring::v1::SystemdJournalResponse* reply) {
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
