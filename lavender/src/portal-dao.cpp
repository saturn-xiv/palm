#include "palm/portal.hpp"

std::pair<uint32_t, uint32_t> palm::portal::dao::paginate(
    palm::portal::v1::Page* page, palm::portal::v1::Pagination* pagination,
    uint32_t total) {
  // TODO
}

std::vector<std::string> palm::portal::dao::locales::languages(
    soci::session& db) {
  // TODO
}
void palm::portal::dao::locales::create(soci::session& db,
                                        const std::string& lang,
                                        const std::string& code,
                                        const std::string& message) {
  // TODO
}
void palm::portal::dao::locales::update(soci::session& db, uint32_t id,
                                        const std::string& message) {
  // TODO
}
uint32_t palm::portal::dao::locales::count(soci::session& db) {
  uint32_t c;
  db << R"SQL(SELECT COUNT(*) FROM locales)SQL", soci::into(c);
  return c;
}
std::vector<palm::portal::dao::locales::Item> palm::portal::dao::locales::index(
    soci::session& db, uint32_t offset, uint32_t limit) {
  // TODO
}
std::vector<palm::portal::dao::locales::Item>
palm::portal::dao::locales::by_lang(soci::session& db,
                                    const std::string& lang) {
  std::vector<palm::portal::dao::locales::Item> items;
  //   TODO
  //   soci::rowset<
  //       boost::tuple<uint32_t, std::string, std::string, std::string,
  //       std::tm>> rs =
  //           (db.prepare
  //                << R"SQL(SELECT id, lang, code, message, updated_at FROM
  //                locales WHERE lang = :lang ORDER BY code ASC)SQL",
  //            soci::use(lang, "lang"));
  //   for (auto row = rs.begin(); row != rs.end(); ++row) {
  //     palm::portal::dao::locales::Item it;
  //     boost::tie(it.id, it.lang, it.code, it.message, it.updated_at) = *row;
  //     items.push_back(it);
  //   }
  return items;
}
