#include "palm/auth.hpp"
#include "palm/crypto.hpp"
#include "palm/validator.hpp"

void palm::auth::User::verify(const std::string& password) {
  std::vector<uint8_t> p(password.begin(), password.end());
  std::vector<uint8_t> s(this->salt.begin(), this->salt.end());
  const auto buf = palm::Hmac::instance().sha512(p, s);
  std::string pwd(buf.begin(), buf.end());
  if (pwd != this->password) {
    throw std::runtime_error("bad password");
  }
}

grpc::Status palm::auth::UserService::SignIn(
    grpc::ServerContext* context, const palm::auth::v1::SignInRequest* request,
    palm::auth::v1::SignInResponse* reply) {
  const auto who = request->who();
  std::optional<palm::auth::User> user;
  if (who.has_email()) {
    std::string it = who.email();
    user = this->get_user_by_email(it);
  }
  if (!user && who.has_nick_name()) {
    std::string it = who.nick_name();
    user = this->get_user_by_nick_name(it);
  }
  user->available();
  user->verify(request->password());
  {
    const auto& query = palm::orm::Query::instance();

    palm::orm::PooledConnection con;
    auto db = con.get();

    soci::transaction tr(*db);
    const std::string level =
        palm::auth::v1::LogList_Level_Name(palm::auth::v1::LogList_Level_INFO);
    const std::string ip = context->peer();
    const std::string message = "Sign In.";
    (*db) << query.get("users.sign-in"), soci::use(user->id, "id"),
        soci::use(ip, "ip");
    (*db) << query.get("logs.create"), soci::use(level, "level"),
        soci::use(user->id, "user_id"), soci::use(ip, "ip"),
        soci::use(message, "message");
    tr.commit();
  }
  const std::string token = palm::Jwt::instance().encode(
      user->uid, ACTION_SIGN_IN, {}, std::chrono::days(1));
  reply->set_token(token);
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::SignUp(
    grpc::ServerContext* context, const palm::auth::v1::SignUpRequest* request,
    google::protobuf::Empty* reply) {
  std::string email = request->email();
  if (this->get_user_by_email(email)) {
    std::stringstream ss;
    ss << "user " << email << " already exists!";
    throw std::runtime_error(ss.str());
  }
  std::string nick_name = request->nick_name();
  if (this->get_user_by_nick_name(nick_name)) {
    std::stringstream ss;
    ss << "user " << nick_name << " already exists!";
    throw std::runtime_error(ss.str());
  }

  const auto& query = palm::orm::Query::instance();
  std::optional<palm::auth::User> user;

  {
    palm::orm::PooledConnection con;
    auto db = con.get();

    std::string real_name = request->real_name();
    {
      boost::algorithm::trim(real_name);
      palm::validator::string(real_name, 1, User::REAL_NAME_MAX);
    }

    std::vector<uint8_t> salt_b = palm::random::bytes(palm::Aes::BLOCK_SIZE);
    const std::string salt(salt_b.begin(), salt_b.end());

    const std::string passwd_r = request->password();
    const std::vector<uint8_t> passwd_rb(passwd_r.begin(), passwd_r.end());
    const auto passwd_b = palm::Hmac::instance().sha512(passwd_rb, salt_b);
    const std::string password(passwd_b.begin(), passwd_b.end());
    nlohmann::json profile_j;
    const std::string profile = profile_j.dump();
    const auto uid = palm::uuid();
    const auto logo = palm::gravatar(email);
    const auto time_zone =
        request->has_time_zone() ? request->time_zone() : "UTC";
    const auto lang = request->has_time_zone() ? request->time_zone() : "en-US";
    const std::string ip = context->peer();
    const std::string message = "Sign up.";
    const std::string level =
        palm::auth::v1::LogList_Level_Name(palm::auth::v1::LogList_Level_INFO);
    {
      soci::transaction tr(*db);
      (*db) << query.get("users.sign-up"), soci::use(nick_name, "nick_name"),
          soci::use(real_name, "real_name"), soci::use(email, "email"),
          soci::use(uid, "uid"), soci::use(logo, "logo"),
          soci::use(lang, "lang"), soci::use(time_zone, "time_zone"),
          soci::use(profile, "profile");
      (*db) << query.get("users.by-email"), soci::use(email), soci::into(user);
      (*db) << query.get("logs.create"), soci::use(level, "level"),
          soci::use(user->id, "user_id"), soci::use(ip, "ip"),
          soci::use(message, "message");
      tr.commit();
    }
  }

  {
    auto it = user.value();
    this->send_email(it, ACTION_CONFIRM);
  }
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
    palm::auth::v1::SelfResponse* reply) {
  // TODO renew token if in 5 minutes
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

void palm::auth::UserService::send_email(const palm::auth::User& user,
                                         const std::string& action) {
  //  TODO
}

std::optional<palm::auth::User> palm::auth::UserService::get_user_by_email(
    const std::string& email) {
  std::optional<User> user;
  palm::orm::PooledConnection con;
  auto db = con.get();
  const auto& query = palm::orm::Query::instance();
  std::string it = boost::algorithm::to_lower_copy(email);
  boost::algorithm::trim(it);
  (*db) << query.get("users.by-email"), soci::use(it), soci::into(user);
  return user;
}
std::optional<palm::auth::User> palm::auth::UserService::get_user_by_uid(
    const std::string& uid) {
  std::optional<User> user;
  palm::orm::PooledConnection con;
  auto db = con.get();
  const auto& query = palm::orm::Query::instance();
  (*db) << query.get("users.by-uid"), soci::use(uid), soci::into(user);
  return user;
}
std::optional<palm::auth::User> palm::auth::UserService::get_user_by_nick_name(
    const std::string& nick_name) {
  std::optional<User> user;
  palm::orm::PooledConnection con;
  auto db = con.get();
  const auto& query = palm::orm::Query::instance();
  std::string it = boost::algorithm::to_lower_copy(nick_name);
  boost::algorithm::trim(it);
  (*db) << query.get("users.by-nick_name"), soci::use(it), soci::into(user);
  return user;
}
