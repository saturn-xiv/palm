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
    if (who.has_email()) {
      std::string it = who.email();
      user = palm::auth::dao::user::by_email(*db, it);
    }
    if (!user && who.has_nick_name()) {
      std::string it = who.nick_name();
      user = palm::auth::dao::user::by_nick_name(*db, it);
    }
    user->available();
    user->verify(request->password());

    soci::transaction tr(*db);
    palm::auth::dao::user::sign_in(*db, context, user->id);
    tr.commit();

    const std::string token = palm::Jwt::instance().encode(
        user->uid, ACTION_SIGN_IN, {}, std::chrono::hours(24));  // FIXME days
    reply->set_token(token);
  }
  return grpc::Status::OK;
}
grpc::Status palm::auth::UserService::SignUp(
    grpc::ServerContext* context, const palm::auth::v1::SignUpRequest* request,
    google::protobuf::Empty* reply) {
  {
    palm::orm::PooledConnection con;
    auto db = con.get();

    soci::transaction tr(*db);
    auto user = palm::auth::dao::user::sign_up(*db, context, request);
    tr.commit();
    this->send_email(user, ACTION_CONFIRM);
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

void palm::auth::dao::user::sign_in(soci::session& db,
                                    grpc::ServerContext* context,
                                    int64_t user) {
  const std::string ip = context->peer();
  const std::string message = "Sign In.";
  db << palm::orm::Query::instance().get("users.sign-in"),
      soci::use(user, "id"), soci::use(ip, "ip");
  palm::auth::dao::log::create(db, context, user, message);
}

palm::auth::User palm::auth::dao::user::sign_up(
    soci::session& db, grpc::ServerContext* context,
    const palm::auth::v1::SignUpRequest* request) {
  std::string email = request->email();
  {
    boost::algorithm::trim(email);
    boost::algorithm::to_lower(email);
    palm::validator::email(email);
  }
  if (palm::auth::dao::user::by_email(db, email)) {
    std::stringstream ss;
    ss << "user " << email << " already exists!";
    throw std::runtime_error(ss.str());
  }
  std::string nick_name = request->nick_name();
  {
    boost::algorithm::trim(nick_name);
    boost::algorithm::to_lower(nick_name);
    palm::validator::string(nick_name, 1, User::REAL_NAME_MAX);
  }
  if (palm::auth::dao::user::by_nick_name(db, nick_name)) {
    std::stringstream ss;
    ss << "user " << nick_name << " already exists!";
    throw std::runtime_error(ss.str());
  }

  std::string real_name = request->real_name();
  {
    boost::algorithm::trim(real_name);
    palm::validator::string(real_name, 1, User::REAL_NAME_MAX);
  }

  {
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
    const std::string message = "Sign up.";
    {
      db << palm::orm::Query::instance().get("users.sign-up"),
          soci::use(nick_name, "nick_name"), soci::use(real_name, "real_name"),
          soci::use(email, "email"), soci::use(uid, "uid"),
          soci::use(logo, "logo"), soci::use(lang, "lang"),
          soci::use(time_zone, "time_zone"), soci::use(profile, "profile");
      const auto user = palm::auth::dao::user::by_email(db, email);
      palm::auth::dao::log::create(db, context, user->id, message);
      return user.value();
    }
  }
}
std::optional<palm::auth::User> palm::auth::dao::user::by_uid(
    soci::session& db, const std::string& uid) {
  std::optional<palm::auth::User> it;
  db << palm::orm::Query::instance().get("users.by-uid"), soci::use(uid),
      soci::into(it);
  return it;
}
std::optional<palm::auth::User> palm::auth::dao::user::by_nick_name(
    soci::session& db, const std::string& nick_name) {
  std::string id = boost::algorithm::trim_copy(id);
  boost::algorithm::to_lower(id);

  std::optional<palm::auth::User> it;
  db << palm::orm::Query::instance().get("users.by-nick_name"), soci::use(id),
      soci::into(it);
  return it;
}
std::optional<palm::auth::User> palm::auth::dao::user::by_email(
    soci::session& db, const std::string& email) {
  std::string id = boost::algorithm::trim_copy(id);
  boost::algorithm::to_lower(id);
  std::optional<palm::auth::User> it;
  db << palm::orm::Query::instance().get("users.by-email"), soci::use(id),
      soci::into(it);
  return it;
}
void palm::auth::dao::user::confirm(soci::session& db, int64_t id) {
  db << palm::orm::Query::instance().get("users.confirm"), soci::use(id);
}
void palm::auth::dao::user::lock(soci::session& db, int64_t id) {
  db << palm::orm::Query::instance().get("users.lock"), soci::use(id);
}
void palm::auth::dao::user::unlock(soci::session& db, int64_t id) {
  db << palm::orm::Query::instance().get("users.unlock"), soci::use(id);
}
void palm::auth::dao::user::destroy(soci::session& db, int64_t id) {
  db << palm::orm::Query::instance().get("users.destroy"), soci::use(id);
}
void palm::auth::dao::log::create(soci::session& db,
                                  grpc::ServerContext* context, int64_t user,
                                  const std::string& message,
                                  const std::string& level) {
  const std::string ip = context->peer();
  db << palm::orm::Query::instance().get("logs.create"),
      soci::use(level, "level"), soci::use(user, "user_id"),
      soci::use(ip, "ip"), soci::use(message, "message");
}
