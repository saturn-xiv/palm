#include "palm/nut.hpp"
#include "palm/auth.hpp"

grpc::Status palm::nut::SiteService::About(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::nut::v1::AboutResponse* reply) {
  return grpc::Status::OK;
}

grpc::Status palm::nut::SiteService::Install(
    grpc::ServerContext* context, const palm::auth::v1::SignUpRequest* request,
    google::protobuf::Empty* reply) {
  const auto& query = palm::orm::Query::instance();
  std::optional<palm::auth::User> user;

  {
    palm::orm::PooledConnection con;
    auto db = con.get();
    std::string real_name = request->real_name();
    // {
    //   boost::algorithm::trim(real_name);
    //   palm::validator::string(real_name, 1, User::REAL_NAME_MAX);
    // }

    // std::string email = request.email();
    // std::string nick_name = request.nickname();
    // boost::algorithm::trim(nick_name);
    // boost::algorithm::trim(email);

    // std::vector<uint8_t> salt_b = palm::random::bytes(palm::Aes::BLOCK_SIZE);
    // const std::string salt(salt_b.begin(), salt_b.end());

    // const std::string passwd_r = request->password();
    // const std::vector<uint8_t> passwd_rb(passwd_r.begin(), passwd_r.end());
    // const auto passwd_b = palm::Hmac::instance().sha512(passwd_rb, salt_b);
    // const std::string password(passwd_b.begin(), passwd_b.end());
    // nlohmann::json profile_j;
    // const std::string profile = profile_j.dump();
    // const auto uid = palm::uuid();
    // const auto logo = palm::gravatar(email);
    // const auto time_zone =
    //     request->has_time_zone() ? request->time_zone() : "UTC";
    // const auto lang = request->has_time_zone() ? request->time_zone() :
    // "en-US"; const std::string ip = context->peer(); const std::string
    // message = "Sign up."; const std::string level =
    //     palm::auth::v1::LogList_Level_Name(palm::auth::v1::LogList_Level_INFO);
    // {
    //   soci::transaction tr(*db);
    //   (*db) << query.get("users.sign-up"), soci::use(nick_name, "nick_name"),
    //       soci::use(real_name, "real_name"), soci::use(email, "email"),
    //       soci::use(uid, "uid"), soci::use(logo, "logo"),
    //       soci::use(lang, "lang"), soci::use(time_zone, "time_zone"),
    //       soci::use(profile, "profile");
    //   (*db) << query.get("users.by-email"), soci::use(email),
    //   soci::into(user);
    //   (*db) << query.get("logs.create"), soci::use(level, "level"),
    //       soci::use(user->id, "user_id"), soci::use(ip, "ip"),
    //       soci::use(message, "message");
    //   tr.commit();
    // }
  }
  return grpc::Status::OK;
}
