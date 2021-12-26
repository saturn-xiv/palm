#pragma once

#include "auth.grpc.pb.h"
#include "palm/orm.hpp"

namespace palm {
namespace auth {

struct User {
  static const size_t NICK_NAME_MAX = 32;
  static const size_t REAL_NAME_MAX = 64;
  inline void available() {
    std::stringstream ss;
    ss << "user " << this->real_name << " ";
    if (!this->confirmed_at) {
      ss << "didn't confirmed";
      throw std::runtime_error(ss.str());
    }
    if (this->locked_at) {
      ss << "didn't confirmed";
      throw std::runtime_error(ss.str());
    }
    if (this->deleted_at) {
      ss << "is deleted";
      throw std::runtime_error(ss.str());
    }
  }
  void verify(const std::string& password);

  int64_t id;
  std::string real_name;
  std::string nick_name;
  std::string email;
  std::optional<std::string> password;
  std::string salt;
  std::string uid;
  std::string logo;
  std::string lang;
  std::string time_zone;
  std::optional<std::tm> confirmed_at;
  std::optional<std::tm> locked_at;
  std::optional<std::tm> deleted_at;
  std::tm updated_at;
};
class UserService final : public palm::auth::v1::User::Service {
 public:
  grpc::Status SignIn(grpc::ServerContext* context,
                      const palm::auth::v1::SignInRequest* request,
                      palm::auth::v1::SignInResponse* reply) override;
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
                    palm::auth::v1::SelfResponse* reply) override;
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

 private:
  void send_email(const User& user, const std::string& action);

  std::optional<User> get_user_by_email(const std::string& email);
  std::optional<User> get_user_by_uid(const std::string& uid);
  std::optional<User> get_user_by_nick_name(const std::string& nick_name);

  inline static const std::string ACTION_SIGN_IN = "user.sign-in";
  inline static const std::string ACTION_CONFIRM = "user.confirm";
  inline static const std::string ACTION_UNLOCK = "user.unlock";
  inline static const std::string ACTION_RESET_PASSWORD = "user.reset-password";
};

}  // namespace auth
}  // namespace palm

namespace soci {
template <>
struct type_conversion<palm::auth::User> {
  typedef soci::values base_type;

  static void from_base(const soci::values& v, soci::indicator i,
                        palm::auth::User& o) {
    o.id = v.get<int64_t>("id");
    o.real_name = v.get<std::string>("real_name");
    o.nick_name = v.get<std::string>("nick_name");
    o.email = v.get<std::string>("email");
    o.password = v.get<std::optional<std::string>>("password");
    o.salt = v.get<std::string>("salt");
    o.uid = v.get<std::string>("uid");
    o.logo = v.get<std::string>("logo");
    o.lang = v.get<std::string>("lang");
    o.time_zone = v.get<std::string>("time_zone");
    o.confirmed_at = v.get<std::optional<std::tm>>("confirmed_at");
    o.locked_at = v.get<std::optional<std::tm>>("locked_at");
    o.deleted_at = v.get<std::optional<std::tm>>("deleted_at");
    o.updated_at = v.get<std::tm>("updated_at");
  }
};
}  // namespace soci
