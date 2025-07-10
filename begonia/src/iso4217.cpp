#include "palm/iso4217.hpp"

#include <pugixml.hpp>

static std::vector<palm::iso4217::Currency> parse_list_one_xml(
    const std::filesystem::path& file) {
  spdlog::info("load iso4217 from {}", file.string());

  pugi::xml_document doc;
  pugi::xml_parse_result result = doc.load_file(file.c_str());
  if (!result) {
    spdlog::error("{}", result.description());
    return {};
  }
  std::vector<palm::iso4217::Currency> items;
  pugi::xpath_node_set nodes = doc.select_nodes("/ISO_4217/CcyTbl/CcyNtry");

  for (pugi::xpath_node node : nodes) {
    pugi::xml_node it = node.node();
    palm::iso4217::Currency currency;

    currency.name = it.child("CcyNm").text().as_string();
    currency.country = it.child("CtryNm").text().as_string();
    currency.code = it.child("Ccy").text().as_string();
    currency.number = it.child("CcyNbr").text().as_string();
    {
      auto node = it.child("CcyMnrUnts");
      if (!node) {
        spdlog::warn("skip ({},{},{},{})", currency.code, currency.name,
                     currency.country, currency.number);
        continue;
      }
      std::string units = node.text().as_string();
      if (units != "N.A.") {
        currency.units = std::stoi(units);
      }
    }
    {
      auto att = it.child("CcyNm").attribute("IsFund");
      if (att) {
        currency.is_fund = att.as_bool() ? 1 : 0;
      }
    }

    items.push_back(currency);
  }
  return items;
}
void palm::iso4217::load(soci::session& db,
                         const std::filesystem::path& list_one_xml) {
  const auto items = parse_list_one_xml(list_one_xml);
  if (palm::iso4217::total(db) == 0) {
    for (auto const& it : items) {
      spdlog::debug("found currency({}, {}, {}, {})", it.code, it.name,
                    it.country, it.number);
      db << R"SQL(INSERT INTO currencies(name, code, number, country, units, is_fund) VALUES(:name, :code, :number, :country, :units, :is_fund))SQL",
          soci::use(it);
    }
  } else {
    spdlog::warn("table is't empty, skipped");
  }
  spdlog::info("found {} currencies", items.size());
}

boost::fusion::vector<palm::iso4217::Currency> palm::iso4217::all(
    soci::session& db) {
  boost::fusion::vector<palm::iso4217::Currency> items;
  db << R"SQL(SELECT id, code, name, country, number, units, is_fund FROM currencies ORDER BY country ASC)SQL",
      soci::into(items);
  return items;
}

boost::optional<palm::iso4217::Currency> palm::iso4217::get(soci::session& db,
                                                            int id) {
  boost::optional<palm::iso4217::Currency> it;
  db << R"SQL(SELECT id, code, name, country, number, units, is_fund FROM currencies WHERE id = :id LIMIT 1)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}

int palm::iso4217::total(soci::session& db) {
  int c = 0;
  db << R"SQL(SELECT COUNT(*) FROM currencies)SQL", soci::into(c);
  return c;
}
