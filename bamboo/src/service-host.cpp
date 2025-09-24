#include "bamboo/models.hpp"
#include "bamboo/services.hpp"

grpc::Status bamboo::services::HostServiceImpl::Index(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::router::v1::HostIndexResponse* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::HostServiceImpl::SetDescription(
    grpc::ServerContext* context,
    const palm::router::v1::HostSetDescriptionRequest* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO

  return grpc::Status::OK;
}
grpc::Status bamboo::services::HostServiceImpl::Enable(
    grpc::ServerContext* context, const palm::portal::v1::ByIdRequest* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::HostServiceImpl::Disable(
    grpc::ServerContext* context, const palm::portal::v1::ByIdRequest* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::HostServiceImpl::Block(
    grpc::ServerContext* context,
    const palm::router::v1::HostBlockRequest* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::HostServiceImpl::SetStaticIpAddress(
    grpc::ServerContext* context,
    const palm::router::v1::HostSetStaticIpAddressRequest* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::HostServiceImpl::SetDhcpAddress(
    grpc::ServerContext* context,
    const palm::router::v1::HostSetDhcpAddressRequest* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
