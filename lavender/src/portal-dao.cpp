#include "palm/portal.hpp"

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
