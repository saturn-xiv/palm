#pragma once

#include "palm/orm.hpp"

#include <filesystem>

namespace palm {
namespace iso4217 {
struct Currency {
  int id;
  std::string name;
  std::string code;
  std::string number;
  std::string country;
  boost::optional<int> units;
  boost::optional<int> is_fund;
};
// https://www.iso.org/iso-4217-currency-codes.html
void load(soci::session& db, const std::filesystem::path& list_one_xml);
boost::fusion::vector<Currency> all(soci::session& db);
}  // namespace iso4217
}  // namespace palm

namespace soci {
template <>
struct type_conversion<palm::iso4217::Currency> {
  typedef soci::values base_type;

  static void from_base(soci::values const& v, soci::indicator /* ind */,
                        palm::iso4217::Currency& p) {
    p.id = v.get<int>("id");
    p.name = v.get<int>("name");
    p.code = v.get<std::string>("code");
    p.number = v.get<std::string>("number");
    p.country = v.get<std::string>("country");
    p.units = v.get<boost::optional<int>>("units");
    p.is_fund = v.get<boost::optional<int>>("is_fund");
  }

  static void to_base(const palm::iso4217::Currency& p, soci::values& v,
                      soci::indicator& ind) {
    v.set("id", p.id);
    v.set("name", p.name);
    v.set("code", p.code);
    v.set("number", p.number);
    v.set("country", p.country);
    v.set("units", p.units);
    v.set("is_fund", p.is_fund);
    ind = i_ok;
  }
};
}  // namespace soci
