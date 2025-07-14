#pragma once

#include "palm/http.hpp"

#include <google/protobuf/util/json_util.h>
#include <google/protobuf/util/time_util.h>
#include <httplib.h>
#include <inja/inja.hpp>

namespace palm {

class Theme {
 public:
  Theme(const std::string& folder, const nlohmann::json& global = {})
      : _env({std::format("{}/views/", folder)}), _global(global) {}

  inline std::string render(const std::string& tpl, nlohmann::json& data) {
    data.update(this->_global);
    return this->_env.render_file(tpl, data);
  }

 private:
  inja::Environment _env;
  nlohmann::json _global;
};

void set_logger(httplib::Server& server);
void tm2ts(std::tm* time, google::protobuf::Timestamp* timestamp);
void str2ts(const std::string& time, google::protobuf::Timestamp* timestamp);
/*
PostgreSQL: timestamp without time zone
2025-07-13 10:49:04.782031+00
*/
std::optional<std::string> to_json(const google::protobuf::Message& message);
}  // namespace palm
