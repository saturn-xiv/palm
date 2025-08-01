#include "lavender/portal.hpp"

#include <sodium.h>

// ----------------------------------------------------------------------------

boost::optional<lavender::portal::dao::users::Item>
lavender::portal::dao::users::get(soci::session& db, int id) {
  boost::optional<lavender::portal::dao::users::Item> it;
  db << R"SQL(SELECT id, uid, lang, timezone, sign_in_count, current_sign_in_at, current_sign_in_ip, last_sign_in_at, last_sign_in_ip, locked_at, deleted_at, version, updated_at FROM users WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
boost::optional<lavender::portal::dao::users::Item>
lavender::portal::dao::users::get(soci::session& db, const std::string& uid) {
  boost::optional<lavender::portal::dao::users::Item> it;
  db << R"SQL(SELECT id, uid, lang, timezone, sign_in_count, current_sign_in_at, current_sign_in_ip, last_sign_in_at, last_sign_in_ip, locked_at, deleted_at, version, updated_at FROM users WHERE uid = :uid LIMIT 1)SQL",
      soci::use(uid, "uid"), soci::into(it);
  return it;
}
boost::fusion::vector<lavender::portal::dao::users::Item>
lavender::portal::dao::users::all(soci::session& db) {
  boost::fusion::vector<lavender::portal::dao::users::Item> items;
  db << R"SQL(SELECT id, uid, lang, timezone, sign_in_count, current_sign_in_at, current_sign_in_ip, last_sign_in_at, last_sign_in_ip, locked_at, deleted_at, version, updated_at FROM users ORDER BY updated_at DESC)SQL",
      soci::into(items);
  return items;
}
void lavender::portal::dao::users::enable(soci::session& db, int id) {
  db << R"SQL(UPDATE users SET deleted_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void lavender::portal::dao::users::disable(soci::session& db, int id) {
  db << R"SQL(UPDATE users SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void lavender::portal::dao::users::create(soci::session& db,
                                          const std::string& uid,
                                          const std::string& lang,
                                          const std::string& timezone) {
  db << R"SQL(INSERT INTO users(uid, lang, timezone, updated_at) VALUES(:uid, :lang, :timezone, CURRENT_TIMESTAMP))SQL",
      soci::use(uid, "uid"), soci::use(lang, "lang"),
      soci::use(timezone, "timezone");
}
// ----------------------------------------------------------------------------

static inline bool verify_email_user_password(const std::string& code,
                                              const std::string& plain) {
  if (code.length() != crypto_pwhash_STRBYTES) {
    return false;
  }
  return crypto_pwhash_str_verify(code.c_str(), plain.c_str(),
                                  plain.length()) == 0;
}
static inline std::optional<std::string> build_email_user_password(
    const std::string& plain) {
  char code[crypto_pwhash_STRBYTES];

  if (crypto_pwhash_str(code, plain.c_str(), plain.length(),
                        crypto_pwhash_OPSLIMIT_SENSITIVE,
                        crypto_pwhash_MEMLIMIT_SENSITIVE) != 0) {
    spdlog::error("failed to hash password");
    return std::nullopt;
  }
  return std::string(code);
}

void lavender::portal::dao::users::email::create(soci::session& db, int user_id,
                                                 const std::string& real_name,
                                                 const std::string& email,
                                                 const std::string& password) {
  const auto code = build_email_user_password(password);
  const auto avatar = palm::gravatar::image(email);
  db << R"SQL(INSERT INTO email_users(user_id, real_name, email, password, avatar, updated_at) VALUES(:user_id, :real_name, :email, :password, :avatar, CURRENT_TIMESTAMP))SQL",
      soci::use(user_id, "user_id"), soci::use(real_name, "real_name"),
      soci::use(email, "email"), soci::use(code.value(), "password"),
      soci::use(avatar, "avatar");
}
void lavender::portal::dao::users::email::set_password(
    soci::session& db, int id, const std::string& password) {
  const auto code = build_email_user_password(password);
  db << R"SQL(UPDATE email_users SET password = :password, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id"), soci::use(code.value(), "password");
}
void lavender::portal::dao::users::email::confirm(soci::session& db, int id) {
  db << R"SQL(UPDATE email_users SET confirmed_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}

boost::optional<lavender::portal::dao::users::email::Item>
lavender::portal::dao::users::email::get(soci::session& db, int id) {
  boost::optional<lavender::portal::dao::users::email::Item> it;
  db << R"SQL(SELECT id, user_id, real_name, email, password, avatar, confirmed_at, deleted_at, version, updated_at FROM email_users WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
boost::optional<lavender::portal::dao::users::email::Item>
lavender::portal::dao::users::email::get(soci::session& db,
                                         const std::string& email) {
  boost::optional<lavender::portal::dao::users::email::Item> it;
  db << R"SQL(SELECT id, user_id, real_name, email, password, avatar, confirmed_at, deleted_at, version, updated_at FROM email_users WHERE email = :email LIMIT 1)SQL",
      soci::use(email, "email"), soci::into(it);
  return it;
}
void lavender::portal::dao::users::email::enable(soci::session& db, int id) {
  db << R"SQL(UPDATE email_users SET deleted_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void lavender::portal::dao::users::email::disable(soci::session& db, int id) {
  db << R"SQL(UPDATE email_users SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
boost::fusion::vector<lavender::portal::dao::users::email::Item>
lavender::portal::dao::users::email::all(soci::session& db) {
  boost::fusion::vector<lavender::portal::dao::users::email::Item> items;
  db << R"SQL(SELECT id, user_id, real_name, email, password, avatar, confirmed_at, deleted_at, version, updated_at FROM email_users ORDER BY real_name ASC)SQL",
      soci::into(items);
  return items;
}
// ----------------------------------------------------------------------------

boost::optional<lavender::portal::dao::users::wechat::mini_program::Item>
lavender::portal::dao::users::wechat::mini_program::get(soci::session& db,
                                                        int id) {
  boost::optional<lavender::portal::dao::users::wechat::mini_program::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, avatar_url, deleted_at, version, updated_at FROM wechat_mini_program_users WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
boost::fusion::vector<lavender::portal::dao::users::wechat::mini_program::Item>
lavender::portal::dao::users::wechat::mini_program::all(soci::session& db) {
  boost::fusion::vector<
      lavender::portal::dao::users::wechat::mini_program::Item>
      items;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, avatar_url, deleted_at, version, updated_at FROM wechat_mini_program_users ORDER BY nickname ASC)SQL",
      soci::into(items);
  return items;
}
void lavender::portal::dao::users::wechat::mini_program::enable(
    soci::session& db, int id) {
  db << R"SQL(UPDATE wechat_mini_program_users SET deleted_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void lavender::portal::dao::users::wechat::mini_program::disable(
    soci::session& db, int id) {
  db << R"SQL(UPDATE wechat_mini_program_users SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
boost::optional<lavender::portal::dao::users::wechat::mini_program::Item>
lavender::portal::dao::users::wechat::mini_program::get(
    soci::session& db, const std::string& union_id) {
  boost::optional<lavender::portal::dao::users::wechat::mini_program::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, avatar_url, deleted_at, version, updated_at FROM wechat_mini_program_users WHERE union_id = :union_id LIMIT 1)SQL",
      soci::use(union_id, "union_id"), soci::into(it);
  return it;
}
boost::optional<lavender::portal::dao::users::wechat::mini_program::Item>
lavender::portal::dao::users::wechat::mini_program::get(
    soci::session& db, const std::string& app_id, const std::string& open_id) {
  boost::optional<lavender::portal::dao::users::wechat::mini_program::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, avatar_url, deleted_at, version, updated_at FROM wechat_mini_program_users WHERE app_id = :app_id AND open_id = :open_id LIMIT 1)SQL",
      soci::use(app_id, "app_id"), soci::use(open_id, "open_id"),
      soci::into(it);
  return it;
}
// ----------------------------------------------------------------------------

boost::optional<lavender::portal::dao::users::wechat::oauth2::Item>
lavender::portal::dao::users::wechat::oauth2::get(soci::session& db, int id) {
  boost::optional<lavender::portal::dao::users::wechat::oauth2::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, sex, city, province, country, head_img_url, privilege, lang, deleted_at, version, updated_at FROM wechat_oauth2_users WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
boost::fusion::vector<lavender::portal::dao::users::wechat::oauth2::Item>
lavender::portal::dao::users::wechat::oauth2::all(soci::session& db) {
  boost::fusion::vector<lavender::portal::dao::users::wechat::oauth2::Item>
      items;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, sex, city, province, country, head_img_url, privilege, lang, deleted_at, version, updated_at FROM wechat_oauth2_users ORDER BY nickname ASC)SQL",
      soci::into(items);
  return items;
}
boost::optional<lavender::portal::dao::users::wechat::oauth2::Item>
lavender::portal::dao::users::wechat::oauth2::get(soci::session& db,
                                                  const std::string& union_id) {
  boost::optional<lavender::portal::dao::users::wechat::oauth2::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, sex, city, province, country, head_img_url, privilege, lang, deleted_at, version, updated_at FROM wechat_oauth2_users WHERE union_id = :union_id LIMIT 1)SQL",
      soci::use(union_id, "union_id"), soci::into(it);
  return it;
}
boost::optional<lavender::portal::dao::users::wechat::oauth2::Item>
lavender::portal::dao::users::wechat::oauth2::get(soci::session& db,
                                                  const std::string& app_id,
                                                  const std::string& open_id) {
  boost::optional<lavender::portal::dao::users::wechat::oauth2::Item> it;
  db << R"SQL(SELECT id, user_id, union_id, app_id, open_id, nickname, sex, city, province, country, head_img_url, privilege, lang, deleted_at, version, updated_at FROM wechat_oauth2_users WHERE app_id = :app_id AND open_id = :open_id LIMIT 1)SQL",
      soci::use(app_id, "app_id"), soci::use(open_id, "open_id"),
      soci::into(it);
  return it;
}
void lavender::portal::dao::users::wechat::oauth2::enable(soci::session& db,
                                                          int id) {
  db << R"SQL(UPDATE wechat_oauth2_users SET deleted_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void lavender::portal::dao::users::wechat::oauth2::disable(soci::session& db,
                                                           int id) {
  db << R"SQL(UPDATE wechat_oauth2_users SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
// ----------------------------------------------------------------------------
boost::optional<lavender::portal::dao::users::google::oauth2::Item>
lavender::portal::dao::users::google::oauth2::get(soci::session& db, int id) {
  boost::optional<lavender::portal::dao::users::google::oauth2::Item> it;
  db << R"SQL(SELECT id, user_id, subject, email, email_verified, name, picture, locale, deleted_at, version, updated_at FROM google_oauth2_users WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
boost::fusion::vector<lavender::portal::dao::users::google::oauth2::Item>
lavender::portal::dao::users::google::oauth2::all(soci::session& db) {
  boost::fusion::vector<lavender::portal::dao::users::google::oauth2::Item>
      items;
  db << R"SQL(SELECT id, user_id, subject, email, email_verified, name, picture, locale, deleted_at, version, updated_at FROM google_oauth2_users ORDER BY updated_at DESC)SQL",
      soci::into(items);
  return items;
}
void lavender::portal::dao::users::google::oauth2::enable(soci::session& db,
                                                          int id) {
  db << R"SQL(UPDATE google_oauth2_users SET deleted_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
void lavender::portal::dao::users::google::oauth2::disable(soci::session& db,
                                                           int id) {
  db << R"SQL(UPDATE google_oauth2_users SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id");
}
// ----------------------------------------------------------------------------
std::vector<std::string> lavender::portal::dao::locales::languages(
    soci::session& db) {
  std::vector<std::string> items;
  db << R"SQL(SELECT DISTINCT lang FROM locales)SQL", soci::into(items);
  return items;
}
void lavender::portal::dao::locales::create(soci::session& db,
                                            const std::string& lang,
                                            const std::string& code,
                                            const std::string& message) {
  db << R"SQL(INSERT INTO locales(lang, code, message, updated_at) VALUES(:lang, :code, :message, CURRENT_TIMESTAMP))SQL",
      soci::use(code, "code"), soci::use(lang, "lang"),
      soci::use(message, "message");
}
void lavender::portal::dao::locales::update(soci::session& db, int id,
                                            const std::string& message) {
  db << R"SQL(UPDATE locales SET message = :message, updated_at = CURRENT_TIMESTAMP WHERE id = :id)SQL",
      soci::use(id, "id"), soci::use(message, "message");
}
void lavender::portal::dao::locales::destroy(soci::session& db, int id) {
  db << R"SQL(DELETE FROM locales WHERE id = :id)SQL", soci::use(id, "id");
}
int lavender::portal::dao::locales::count(soci::session& db) {
  int c = 0;
  db << R"SQL(SELECT COUNT(*) FROM locales)SQL", soci::into(c);
  return c;
}
boost::fusion::vector<lavender::portal::dao::locales::Item>
lavender::portal::dao::locales::index(soci::session& db, int offset,
                                      int limit) {
  boost::fusion::vector<lavender::portal::dao::locales::Item> items;
  db << R"SQL(SELECT id, lang, code, message, updated_at FROM locales ORDER BY updated_at DESC OFFSET :offset LIMIT :limit)SQL",
      soci::into(items), soci::use(offset, "offset"), soci::use(limit, "limit");
  return items;
}
boost::fusion::vector<lavender::portal::dao::locales::Item>
lavender::portal::dao::locales::by_lang(soci::session& db,
                                        const std::string& lang) {
  boost::fusion::vector<lavender::portal::dao::locales::Item> items;
  db << R"SQL(SELECT id, lang, code, message, updated_at FROM locales WHERE lang=:lang ORDER BY code ASC)SQL",
      soci::into(items), soci::use(lang, "lang");
  return items;
}

boost::optional<lavender::portal::dao::locales::Item>
lavender::portal::dao::locales::get(soci::session& db, int id) {
  boost::optional<lavender::portal::dao::locales::Item> it;
  db << R"SQL(SELECT id, lang, code, message, updated_at FROM locales WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}
boost::optional<lavender::portal::dao::locales::Item>
lavender::portal::dao::locales::get(soci::session& db, const std::string& lang,
                                    const std::string& code) {
  boost::optional<lavender::portal::dao::locales::Item> it;
  db << R"SQL(SELECT id, lang, code, message, updated_at FROM locales WHERE lang = :lang AND code = :code LIMIT 1)SQL",
      soci::use(lang, "lang"), soci::use(code, "code"), soci::into(it);
  return it;
}

inline static void load_locales_from_toml(
    std::vector<std::tuple<std::string, std::string, std::string>>& items,
    const std::string& lang, const std::string& zone,
    const toml::table& table) {
  for (auto&& [key, val] : table) {
    const std::string code = std::format("{}.{}", zone, key.str());
    if (val.is_string()) {
      const std::string message = val.as_string()->get();
      items.push_back({lang, code, message});
      continue;
    }
    if (val.is_table()) {
      load_locales_from_toml(items, lang, code, *(val.as_table()));
      continue;
    }
    spdlog::warn("unknown field {}", code);
  }
}

inline static void load_locales_from_toml_file(
    std::vector<std::tuple<std::string, std::string, std::string>>& items,
    const std::string& lang, const std::filesystem::path& file) {
  spdlog::debug("load locales for {} from {}", lang, file.string());
  toml::table root = toml::parse_file(file.string());
  load_locales_from_toml(items, lang, file.stem().string(), root);
}

inline static std::vector<std::tuple<std::string, std::string, std::string>>
load_locales_from_toml_folder(const std::filesystem::path& root) {
  spdlog::info("load locales from {}", root.string());
  std::vector<std::tuple<std::string, std::string, std::string>> items;

  std::vector<std::string> languages;
  for (const auto& entry : std::filesystem::directory_iterator(root)) {
    auto it = entry.path();
    if (std::filesystem::is_directory(it)) {
      const auto lang = it.filename().string();
      spdlog::debug("find language {}", lang);
      languages.push_back(lang);
    }
  }
  for (const auto& entry : std::filesystem::directory_iterator(root)) {
    auto it = entry.path();
    if (std::filesystem::is_regular_file(it)) {
      for (const auto& lang : languages) {
        load_locales_from_toml_file(items, lang, it);
      }
    }
  }
  for (const auto& lang : languages) {
    for (const auto& entry : std::filesystem::directory_iterator(root / lang)) {
      auto it = entry.path();
      if (std::filesystem::is_regular_file(it)) {
        load_locales_from_toml_file(items, lang, it);
      }
    }
  }
  return items;
}
void lavender::portal::dao::locales::load(soci::session& db,
                                          const std::filesystem::path& folder) {
  const auto items = load_locales_from_toml_folder(folder);
  int inserted = 0;
  for (const auto& [lang, code, message] : items) {
    auto it = lavender::portal::dao::locales::get(db, lang, code);
    if (!it.is_initialized()) {
      lavender::portal::dao::locales::create(db, lang, code, message);
      inserted++;
    }
  }
  spdlog::info("found {} items, insert {} items", items.size(), inserted);
}
// ----------------------------------------------------------------------------
void lavender::portal::dao::logs::create(
    soci::session& db, int user, const std::string& plugin,
    const std::string& ip,
    palm::portal::v1::UserIndexLogResponse_Item_Level level,
    const std::string& resource_type, boost::optional<int> resource_id,
    const std::string& message) {
  // const std::string level_name =
  //     palm::portal::v1::UserIndexLogResponse_Item_Level_Name(level);
  db << R"SQL(INSERT INTO logs(user_id, plugin, ip, level, resource_type, resource_id, message) VALUES(:user_id, :plugin, :ip, :level, :resource_type, :resource_id, :message))SQL",
      soci::use(user, "user_id"), soci::use(plugin, "plugin"),
      soci::use(ip, "ip"), soci::use(static_cast<int>(level), "level"),
      soci::use(resource_type, "resource_type"),
      soci::use(resource_id, "resource_id"), soci::use(message, "message");
}
boost::fusion::vector<lavender::portal::dao::logs::Item>
lavender::portal::dao::logs::index(soci::session& db, int user, int offset,
                                   int limit) {
  boost::fusion::vector<lavender::portal::dao::logs::Item> items;
  db << R"SQL(SELECT id, user_id, plugin, ip, level, resource_type, resource_id, message, created_at FROM logs WHERE user_id = :user_id ORDER BY created_at DESC OFFSET :offset LIMIT :limit)SQL",
      soci::into(items), soci::use(user, "user_id"),
      soci::use(offset, "offset"), soci::use(limit, "limit");
  return items;
}
// ----------------------------------------------------------------------------
