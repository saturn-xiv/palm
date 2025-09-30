#include "bamboo/services.hpp"

grpc::Status bamboo::services::UserServiceImpl::Index(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::router::v1::UserIndexResponse* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::UserServiceImpl::Create(
    grpc::ServerContext* context,
    const palm::router::v1::UserCreateRequest* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::UserServiceImpl::SetRealName(
    grpc::ServerContext* context,
    const palm::router::v1::UserSetRealNameRequest* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::UserServiceImpl::SetContact(
    grpc::ServerContext* context,
    const palm::router::v1::UserSetContactRequest* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::UserServiceImpl::SetWifi(
    grpc::ServerContext* context,
    const palm::router::v1::UserSetWifiRequest* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  // TODO
  return grpc::Status::OK;
}
