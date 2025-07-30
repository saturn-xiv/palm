#include "palm/services.hpp"

grpc::Status palm::monitoring::services::SystemdServiceImpl::Journal(
    grpc::ServerContext* context,
    const palm::monitoring::v1::SystemdJournalRequest* request,
    palm::monitoring::v1::SystemdJournalResponse* reply) {
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
