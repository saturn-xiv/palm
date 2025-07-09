#include "palm/portal.hpp"

// ----------------------------------------------------------------------------

boost::optional<palm::portal::dao::users::Item> palm::portal::dao::users::get(
    soci::session& db, int id) {
  boost::optional<palm::portal::dao::users::Item> it;
  db << R"SQL(SELECT id, uid, lang, timezone, sign_in_count, current_sign_in_at, current_sign_in_ip, last_sign_in_at, last_sign_in_ip, locked_at, deleted_at, version, updated_at FROM users WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
boost::optional<palm::portal::dao::users::Item> palm::portal::dao::users::get(
    soci::session& db, const std::string& uid) {
  boost::optional<palm::portal::dao::users::Item> it;
  db << R"SQL(SELECT id, uid, lang, timezone, sign_in_count, current_sign_in_at, current_sign_in_ip, last_sign_in_at, last_sign_in_ip, locked_at, deleted_at, version, updated_at FROM users WHERE uid = :uid LIMIT 1)SQL",
      soci::use(uid, "uid"), soci::into(it);
  return it;
}
void palm::portal::dao::users::enable(soci::session& db, int id) {
  db << R"SQL(UPDATE users SET deleted_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void palm::portal::dao::users::disable(soci::session& db, int id) {
  db << R"SQL(UPDATE users SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
// ----------------------------------------------------------------------------

boost::optional<palm::portal::dao::users::email::Item>
palm::portal::dao::users::email::get(soci::session& db, int id) {
  boost::optional<palm::portal::dao::users::email::Item> it;
  db << R"SQL(SELECT id, user_id, real_name, email, password, avatar, confirmed_at, deleted_at, version, updated_at FROM email_users WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
boost::optional<palm::portal::dao::users::email::Item>
palm::portal::dao::users::email::get(soci::session& db,
                                     const std::string& email) {
  boost::optional<palm::portal::dao::users::email::Item> it;
  db << R"SQL(SELECT id, user_id, real_name, email, password, avatar, confirmed_at, deleted_at, version, updated_at FROM email_users WHERE email = :email LIMIT 1)SQL",
      soci::use(email, "email"), soci::into(it);
  return it;
}
void palm::portal::dao::users::email::enable(soci::session& db, int id) {
  db << R"SQL(UPDATE email_users SET deleted_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void palm::portal::dao::users::email::disable(soci::session& db, int id) {
  db << R"SQL(UPDATE email_users SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
// ----------------------------------------------------------------------------

boost::optional<palm::portal::dao::users::wechat::mini_program::Item>
palm::portal::dao::users::wechat::mini_program::get(soci::session& db, int id) {
  boost::optional<palm::portal::dao::users::wechat::mini_program::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, avatar_url, deleted_at, version, updated_at FROM wechat_mini_program_users WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
void palm::portal::dao::users::wechat::mini_program::enable(soci::session& db,
                                                            int id) {
  db << R"SQL(UPDATE wechat_mini_program_users SET deleted_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void palm::portal::dao::users::wechat::mini_program::disable(soci::session& db,
                                                             int id) {
  db << R"SQL(UPDATE wechat_mini_program_users SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
boost::optional<palm::portal::dao::users::wechat::mini_program::Item>
palm::portal::dao::users::wechat::mini_program::get(
    soci::session& db, const std::string& union_id) {
  boost::optional<palm::portal::dao::users::wechat::mini_program::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, avatar_url, deleted_at, version, updated_at FROM wechat_mini_program_users WHERE union_id = :union_id LIMIT 1)SQL",
      soci::use(union_id, "union_id"), soci::into(it);
  return it;
}
boost::optional<palm::portal::dao::users::wechat::mini_program::Item>
palm::portal::dao::users::wechat::mini_program::get(
    soci::session& db, const std::string& app_id, const std::string& open_id) {
  boost::optional<palm::portal::dao::users::wechat::mini_program::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, avatar_url, deleted_at, version, updated_at FROM wechat_mini_program_users WHERE app_id = :app_id AND open_id = :open_id LIMIT 1)SQL",
      soci::use(app_id, "app_id"), soci::use(open_id, "open_id"),
      soci::into(it);
  return it;
}
// ----------------------------------------------------------------------------

boost::optional<palm::portal::dao::users::wechat::oauth2::Item>
palm::portal::dao::users::wechat::oauth2::get(soci::session& db, int id) {
  boost::optional<palm::portal::dao::users::wechat::oauth2::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, sex, city, province, country, head_img_url, privilege, lang, deleted_at, version, updated_at FROM wechat_oauth2_users WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
boost::optional<palm::portal::dao::users::wechat::oauth2::Item>
palm::portal::dao::users::wechat::oauth2::get(soci::session& db,
                                              const std::string& union_id) {
  boost::optional<palm::portal::dao::users::wechat::oauth2::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, sex, city, province, country, head_img_url, privilege, lang, deleted_at, version, updated_at FROM wechat_oauth2_users WHERE union_id = :union_id LIMIT 1)SQL",
      soci::use(union_id, "union_id"), soci::into(it);
  return it;
}
boost::optional<palm::portal::dao::users::wechat::oauth2::Item>
palm::portal::dao::users::wechat::oauth2::get(soci::session& db,
                                              const std::string& app_id,
                                              const std::string& open_id) {
  boost::optional<palm::portal::dao::users::wechat::oauth2::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, sex, city, province, country, head_img_url, privilege, lang, deleted_at, version, updated_at FROM wechat_oauth2_users WHERE app_id = :app_id AND open_id = :open_id LIMIT 1)SQL",
      soci::use(app_id, "app_id"), soci::use(open_id, "open_id"),
      soci::into(it);
  return it;
}
void palm::portal::dao::users::wechat::oauth2::enable(soci::session& db,
                                                      int id) {
  db << R"SQL(UPDATE wechat_oauth2_users SET deleted_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void palm::portal::dao::users::wechat::oauth2::disable(soci::session& db,
                                                       int id) {
  db << R"SQL(UPDATE wechat_oauth2_users SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
// ----------------------------------------------------------------------------
boost::optional<palm::portal::dao::users::google::oauth2::Item>
palm::portal::dao::users::google::oauth2::get(soci::session& db, int id) {
  boost::optional<palm::portal::dao::users::google::oauth2::Item> it;
  db << R"SQL(SELECT id, user_id, subject, email, email_verified, name, picture, locale, deleted_at, version, updated_at FROM google_oauth2_users WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
void palm::portal::dao::users::google::oauth2::enable(soci::session& db,
                                                      int id) {
  db << R"SQL(UPDATE google_oauth2_users SET deleted_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void palm::portal::dao::users::google::oauth2::disable(soci::session& db,
                                                       int id) {
  db << R"SQL(UPDATE google_oauth2_users SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
// ----------------------------------------------------------------------------
std::vector<std::string> palm::portal::dao::locales::languages(
    soci::session& db) {
  std::vector<std::string> items;
  db << R"SQL(SELECT DISTINCT lang FROM locales)SQL", soci::into(items);
  return items;
}
void palm::portal::dao::locales::create(soci::session& db,
                                        const std::string& lang,
                                        const std::string& code,
                                        const std::string& message) {
  db << R"SQL(INSERT INTO locales(lang, code, message) VALUES(:lang, :code, :message, CURRENT_TIMESTAMP))SQL",
      soci::use(code, "code"), soci::use(lang, "lang"),
      soci::use(message, "message");
}
void palm::portal::dao::locales::update(soci::session& db, int id,
                                        const std::string& message) {
  db << R"SQL(UPDATE locales SET message = :message, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id"), soci::use(message, "message");
}
void palm::portal::dao::locales::destroy(soci::session& db, int id) {
  db << R"SQL(DELETE FROM locales WHERE id = :id)SQL", soci::use(id, "id");
}
int palm::portal::dao::locales::count(soci::session& db) {
  int c;
  db << R"SQL(SELECT COUNT(*) FROM locales)SQL", soci::into(c);
  return c;
}
boost::fusion::vector<palm::portal::dao::locales::Item>
palm::portal::dao::locales::index(soci::session& db, int offset, int limit) {
  boost::fusion::vector<palm::portal::dao::locales::Item> items;
  db << R"SQL(SELECT id, lang, code, message, updated_at FROM locales ORDER BY updated_at DESC OFFSET :offset LIMIT :limit)SQL",
      soci::into(items), soci::use(offset, "offset"), soci::use(limit, "limit");
  return items;
}
boost::fusion::vector<palm::portal::dao::locales::Item>
palm::portal::dao::locales::by_lang(soci::session& db,
                                    const std::string& lang) {
  boost::fusion::vector<palm::portal::dao::locales::Item> items;
  db << R"SQL(SELECT id, lang, code, message, updated_at FROM locales WHERE lang=:lang ORDER BY code ASC)SQL",
      soci::into(items), soci::use(lang, "lang");
  return items;
}

boost::optional<palm::portal::dao::locales::Item>
palm::portal::dao::locales::get(soci::session& db, int id) {
  boost::optional<palm::portal::dao::locales::Item> it;
  db << R"SQL(SELECT id, lang, code, message, updated_at FROM locales WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
boost::optional<palm::portal::dao::locales::Item>
palm::portal::dao::locales::get(soci::session& db, const std::string& lang,
                                const std::string& code) {
  boost::optional<palm::portal::dao::locales::Item> it;
  db << R"SQL(SELECT id, lang, code, message, updated_at FROM locales WHERE lang = :lang AND code = :code LIMIT 1)SQL",
      soci::use(lang, "lang"), soci::use(code, "code"), soci::into(it);
  return it;
}

static std::vector<std::tuple<std::string, std::string, std::string>>
parse_locales_from_toml_folder(const std::filesystem::path& root) {
  // TODO
}
void palm::portal::dao::locales::load(soci::session& db,
                                      const std::filesystem::path& folder) {
  const auto items = parse_locales_from_toml_folder(folder);
  // TODO
}
// ----------------------------------------------------------------------------
