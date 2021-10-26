#pragma once

#include "palm/orm.hpp"

namespace palm {

class I18n {
 public:
  I18n(std::shared_ptr<soci::session> sql,
       const std::shared_ptr<palm::orm::Query> query,
       std::shared_ptr<sw::redis::Redis> redis)
      : sql(sql), query(query), redis(redis) {}
  inline std::string tr(const std::string& lang, const std::string& code) {
    // TODO
    return lang + "." + code;
  }

 private:
  std::shared_ptr<soci::session> sql;
  std::shared_ptr<palm::orm::Query> query;
  std::shared_ptr<sw::redis::Redis> redis;
};
}  // namespace palm
