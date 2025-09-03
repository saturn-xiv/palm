#include "bamboo/services.hpp"

grpc::Status bamboo::services::AdministratorServiceImpl::SignIn(
    grpc::ServerContext* context,
    const palm::router::v1::AdministratorSignInRequest* request,
    palm::router::v1::AdministratorSignInResponse* reply) {
  // TODO
  return grpc::Status::OK;
}

grpc::Status bamboo::services::AdministratorServiceImpl::SignOut(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    google::protobuf::Empty* reply) {
  // TODO
  return grpc::Status::OK;
}

grpc::Status bamboo::services::AdministratorServiceImpl::SetPassword(
    grpc::ServerContext* context,
    const palm::router::v1::AdministratorSetPasswordRequest* request,
    google::protobuf::Empty* reply) {
  // TODO
  return grpc::Status::OK;
}
