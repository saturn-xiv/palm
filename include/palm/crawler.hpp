#pragma once

#include "palm/orm.hpp"

namespace palm {
namespace crawler {
class Site {
 public:
  Site() {}

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Site, id, name, url, ttl, version)

 private:
  int8_t id;
  std::string name;
  std::string url;
  std::chrono::minutes ttl;
  int8_t version;
  std::tm created_at;
  std::tm updated_at;
};

class Log {
 private:
  int8_t id;
  int8_t site_id;
  std::string body;
  std::tm created_at;
};

class Dao {
 public:
  Dao(std::shared_ptr<pqxx::work> tr) : tr(tr) {}
  std::string latest(const std::string& name);
  void fetch(const std::string& name);

 private:
  std::shared_ptr<pqxx::work> tr;
};

}  // namespace crawler
}  // namespace palm
