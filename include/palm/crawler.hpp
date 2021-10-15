#pragma once

#include "palm/orm.hpp"

namespace palm {
namespace crawler {
class Site {
 public:
  Site() {}
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Site, name, url, ttl)

 private:
  std::string name;
  std::string url;
  size_t ttl;
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
