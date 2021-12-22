#pragma once

#include "auth.grpc.pb.h"
#include "palm/orm.hpp"

namespace palm {
namespace auth {
class UserService final : public palm::auth::v1::User::Service {
  grpc::Status SignIn(grpc::ServerContext* context,
                      const palm::auth::v1::SignInRequest* request,
                      google::protobuf::Empty* reply) override;
  grpc::Status SignUp(grpc::ServerContext* context,
                      const palm::auth::v1::SignUpRequest* request,
                      google::protobuf::Empty* reply) override;
  grpc::Status Confirm(grpc::ServerContext* context,
                       const palm::auth::v1::UserQuery* request,
                       google::protobuf::Empty* reply) override;
  grpc::Status ConfirmByToken(grpc::ServerContext* context,
                              const palm::auth::v1::TokenForm* request,
                              google::protobuf::Empty* reply) override;
  grpc::Status Unlock(grpc::ServerContext* context,
                      const palm::auth::v1::UserQuery* request,
                      google::protobuf::Empty* reply) override;
  grpc::Status UnlockByToken(grpc::ServerContext* context,
                             const palm::auth::v1::TokenForm* request,
                             google::protobuf::Empty* reply) override;
  grpc::Status ForgotPassword(grpc::ServerContext* context,
                              const palm::auth::v1::UserQuery* request,
                              google::protobuf::Empty* reply) override;
  grpc::Status ResetPassword(
      grpc::ServerContext* context,
      const palm::auth::v1::ResetPasswordRequest* request,
      google::protobuf::Empty* reply) override;
  grpc::Status ChangePassword(
      grpc::ServerContext* context,
      const palm::auth::v1::ChangePasswordRequest* request,
      google::protobuf::Empty* reply) override;
  grpc::Status SetProfile(grpc::ServerContext* context,
                          const palm::auth::v1::ProfileRequest* request,
                          google::protobuf::Empty* reply) override;
  grpc::Status SignOut(grpc::ServerContext* context,
                       const google::protobuf::Empty* request,
                       google::protobuf::Empty* reply) override;
  grpc::Status Self(grpc::ServerContext* context,
                    const google::protobuf::Empty* request,
                    palm::auth::v1::UserList_Item* reply) override;
  grpc::Status Log(grpc::ServerContext* context,
                   const google::protobuf::Duration* request,
                   palm::auth::v1::LogList* reply) override;
  grpc::Status Index(grpc::ServerContext* context,
                     const google::protobuf::Duration* request,
                     palm::auth::v1::UserList* reply) override;
  grpc::Status Show(grpc::ServerContext* context,
                    const palm::auth::v1::UserQuery* request,
                    palm::auth::v1::UserList_Item* reply) override;
  grpc::Status Lock(grpc::ServerContext* context,
                    const palm::auth::v1::UserQuery* request,
                    google::protobuf::Empty* reply) override;
};
}  // namespace auth
}  // namespace palm
