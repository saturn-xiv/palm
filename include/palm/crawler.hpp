#pragma once

#include "palm/orm.hpp"

namespace palm {
namespace crawler {
class Site {
 public:
  Site() {}

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Site, id, name, url, ttl, version, updated_at,
                                 created_at)

 private:
  int64_t id;
  std::string name;
  std::string url;
  std::chrono::minutes ttl;
  int32_t version;
  std::tm created_at;
  std::tm updated_at;
};

class Log {
 public:
  Log() {}
  static Log latest(soci::session& sql, const int64_t site_id);
  static Log insert(soci::session& sql, const int64_t site_id,
                    const std::string& body);

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Log, id, site_id, body, created_at)

 private:
  int64_t id;
  int64_t site_id;
  std::string body;
  std::tm created_at;
};

}  // namespace crawler
}  // namespace palm
