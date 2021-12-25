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
  {
    palm::orm::PooledConnection con;
    auto db = con.get();
    const auto& query = palm::orm::Query::instance();
    if (who.has_email()) {
      std::string email = who.email();
      boost::algorithm::to_lower(email);
      boost::algorithm::trim(email);
      (*db) << query.get("users.by-email"), soci::use(email), soci::into(user);
    }
    if (who.has_nick_name()) {
      std::string nick_name = who.nick_name();
      boost::algorithm::to_lower(nick_name);
      boost::algorithm::trim(nick_name);
      (*db) << query.get("users.by-nick_name"), soci::use(nick_name),
          soci::into(user);
    }

    user->available();
    user->verify(request->password());

    {
      soci::transaction tr(*db);
      const std::string level = palm::auth::v1::LogList_Level_Name(
          palm::auth::v1::LogList_Level_INFO);
      const std::string ip = context->peer();
      const std::string message = "Sign In.";
      (*db) << query.get("users.sign-in"), soci::use(user->id, "id"),
          soci::use(ip, "ip");
      (*db) << query.get("logs.create"), soci::use(level, "level"),
          soci::use(user->id, "user_id"), soci::use(ip, "ip"),
          soci::use(message, "message");
      tr.commit();
    }
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
  const auto& query = palm::orm::Query::instance();
  std::optional<palm::auth::User> user;

  {
    palm::orm::PooledConnection con;
    auto db = con.get();
    {
      boost::algorithm::to_lower(email);
      boost::algorithm::trim(email);
      palm::validator::email(email);
      (*db) << query.get("users.by-email"), soci::use(email), soci::into(user);
      if (user) {
        std::stringstream ss;
        ss << "user " << email << " already exists!";
        throw std::runtime_error(ss.str());
      }
    }
    std::string real_name = request->real_name();
    {
      boost::algorithm::trim(real_name);
      palm::validator::string(real_name, 1, User::REAL_NAME_MAX);
    }
    std::string nick_name = request->nick_name();
    {
      boost::algorithm::to_lower(nick_name);
      boost::algorithm::trim(nick_name);
      palm::validator::string(nick_name, 1, User::NICK_NAME_MAX);
      (*db) << query.get("users.by-nick_name"), soci::use(nick_name),
          soci::into(user);
      if (user) {
        std::stringstream ss;
        ss << "user " << nick_name << " already exists!";
        throw std::runtime_error(ss.str());
      }
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

  // TODO send confirm email

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
