#pragma once

#include "palm/http.hpp"

#include <boost/optional.hpp>

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

namespace nlohmann {
template <typename T>
struct adl_serializer<boost::optional<T>> {
  static void to_json(json& j, const boost::optional<T>& opt) {
    if (opt == boost::none) {
      j = nullptr;
    } else {
      j = *opt;
    }
  }

  static void from_json(const json& j, boost::optional<T>& opt) {
    if (j.is_null()) {
      opt = boost::none;
    } else {
      opt = j.template get<T>();
    }
  }
};

template <typename T>
struct adl_serializer<std::optional<T>> {
  static void to_json(json& j, const std::optional<T>& opt) {
    if (opt == std::nullopt) {
      j = nullptr;
    } else {
      j = *opt;
    }
  }

  static void from_json(const json& j, std::optional<T>& opt) {
    if (j.is_null()) {
      opt = std::nullopt;
    } else {
      opt = j.template get<T>();
    }
  }
};
}  // namespace nlohmann
