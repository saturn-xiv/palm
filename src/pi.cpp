#include "palm/pi.hpp"

grpc::Status palm::pi::TtyService::Write(
    grpc::ServerContext* context, const palm::pi::v1::TtyRequest* request,
    google::protobuf::Empty* response) {
  return grpc::Status::OK;
}
