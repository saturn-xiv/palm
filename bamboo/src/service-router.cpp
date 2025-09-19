#include "bamboo/services.hpp"
#include "palm/utils.hpp"

grpc::Status bamboo::services::RouterServiceImpl::SetEthernet(
    grpc::ServerContext* context,
    const palm::router::v1::RouterIndexEthernetResponse_Item* request,
    google::protobuf::Empty* reply) {
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::RouterServiceImpl::IndexEthernet(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::router::v1::RouterIndexEthernetResponse* reply) {
  return grpc::Status::OK;
}
grpc::Status bamboo::services::RouterServiceImpl::Reboot(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  palm::reboot();
  return grpc::Status::OK;
}
grpc::Status bamboo::services::RouterServiceImpl::Apply(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    google::protobuf::Empty* reply) {
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::RouterServiceImpl::FactoryReset(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    google::protobuf::Empty* reply) {
  // TODO
  return grpc::Status::OK;
}
