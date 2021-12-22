#include "palm/auth.hpp"

grpc::Status palm::auth::UserService::SignIn(
    grpc::ServerContext* context, const palm::auth::v1::SignInRequest* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::SignUp(
    grpc::ServerContext* context, const palm::auth::v1::SignUpRequest* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::Confirm(
    grpc::ServerContext* context, const palm::auth::v1::UserQuery* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::ConfirmByToken(
    grpc::ServerContext* context, const palm::auth::v1::TokenForm* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::Unlock(
    grpc::ServerContext* context, const palm::auth::v1::UserQuery* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::UnlockByToken(
    grpc::ServerContext* context, const palm::auth::v1::TokenForm* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::ForgotPassword(
    grpc::ServerContext* context, const palm::auth::v1::UserQuery* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::ResetPassword(
    grpc::ServerContext* context,
    const palm::auth::v1::ResetPasswordRequest* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::ChangePassword(
    grpc::ServerContext* context,
    const palm::auth::v1::ChangePasswordRequest* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::SetProfile(
    grpc::ServerContext* context, const palm::auth::v1::ProfileRequest* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::SignOut(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::Self(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::auth::v1::UserList_Item* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::Log(
    grpc::ServerContext* context, const google::protobuf::Duration* request,
    palm::auth::v1::LogList* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::Index(
    grpc::ServerContext* context, const google::protobuf::Duration* request,
    palm::auth::v1::UserList* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::Show(
    grpc::ServerContext* context, const palm::auth::v1::UserQuery* request,
    palm::auth::v1::UserList_Item* reply) {
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::Lock(
    grpc::ServerContext* context, const palm::auth::v1::UserQuery* request,
    google::protobuf::Empty* reply) {
  return grpc::Status::OK;
}
