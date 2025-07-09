#include "palm/iso4217.hpp"

static std::vector<palm::iso4217::Currency> parse_list_one_xml(
    const std::filesystem::path& file) {
  spdlog::info("load iso4217 from {}", file.string());
  // TODO
}
void palm::iso4217::load(soci::session& db,
                         const std::filesystem::path& list_one_xml) {
  const auto items = parse_list_one_xml(list_one_xml);
  // TODO
}

boost::fusion::vector<palm::iso4217::Currency> palm::iso4217::all(
    soci::session& db) {
  // TODO
}
