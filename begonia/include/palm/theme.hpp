#pragma once

#include "palm/http.hpp"
#include "palm/session.hpp"
#include "portal.pb.h"

#include <functional>

#include <boost/algorithm/string.hpp>
#include <boost/optional.hpp>

#include <google/protobuf/util/json_util.h>
#include <google/protobuf/util/time_util.h>
#include <grpcpp/support/status.h>
#include <httplib.h>
#include <spdlog/spdlog.h>
#include <inja/inja.hpp>

namespace palm {

inline void page(const httplib::Request& request,
                 palm::portal::v1::Page* page) {
  int index = std::stoi(request.get_param_value("index"));
  int size = std::stoi(request.get_param_value("size"));
  page->set_index(static_cast<uint32_t>(index));
  page->set_size(static_cast<uint32_t>(size));
}
std::pair<uint32_t, uint32_t> paginate(uint32_t total, uint32_t index = 1,
                                       uint32_t size = 60);
inline void paginate(palm::portal::v1::Page* page,
                     palm::portal::v1::Pagination* pagination, uint32_t total) {
  const auto [index, size] = palm::paginate(total, page->index(), page->size());
  pagination->set_total(total);
  pagination->set_index(index);
  pagination->set_size(size);
}

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
inline void now(google::protobuf::Timestamp* timestamp) {
  const auto it = google::protobuf::util::TimeUtil::GetCurrentTime();
  timestamp->set_seconds(it.seconds());
  timestamp->set_nanos(it.nanos());
}
/*
PostgreSQL: timestamp without time zone
2025-07-13 10:49:04.782031+00
*/
inline std::optional<std::string> to_json(
    const google::protobuf::Message& message) {
  std::string buf;
  const auto status =
      google::protobuf::util::MessageToJsonString(message, &buf);
  if (status.ok()) {
    return buf;
  }
  spdlog::error("failed to serialize google message to json {}",
                status.message());
  return std::nullopt;
}

inline grpc::Status from_json(const std::string buffer,
                              google::protobuf::Message* message) {
  const auto status =
      google::protobuf::util::JsonStringToMessage(buffer, message);
  return status.ok()
             ? grpc::Status::OK
             : grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "invalid json");
}

namespace http {

struct GRpcHandler {
  std::string package;
  std::string service;
  std::string method;
  std::function<std::pair<grpc::Status,
                          std::shared_ptr<google::protobuf::Message>>(
      std::shared_ptr<grpc::Channel>, grpc::ClientContext&, const std::string&)>
      handler;

  bool operator<(const GRpcHandler& other) const {
    if (package != other.package) {
      return package < other.package;
    }
    if (service != other.service) {
      return service < other.service;
    }
    return method < other.method;
  }
};
inline void text(httplib::Response& response, const std::string& content = "",
                 int status = httplib::StatusCode::OK_200) {
  response.set_content(content, palm::http::content_type::TEXT_PLAIN_UTF8);
  response.status = status;
}
inline void abort(httplib::Response& response, const std::string& content = "",
                  int status = httplib::StatusCode::InternalServerError_500) {
  text(response, content, status);
}
inline void abort(httplib::Response& response, const grpc::Status& status,
                  int code = httplib::StatusCode::InternalServerError_500) {
  const std::string content = status.error_message();
  spdlog::error("{}", content);
  abort(response, content, code);
}
inline void abort(httplib::Response& response, const absl::Status& status,
                  int code = httplib::StatusCode::InternalServerError_500) {
  const std::string content(status.message());
  spdlog::error("{} {}", status.raw_code(), content);
  abort(response, content, code);
}
inline void json(httplib::Response& response,
                 const google::protobuf::Message& message,
                 int status = httplib::StatusCode::OK_200) {
  std::string content;
  const auto it =
      google::protobuf::util::MessageToJsonString(message, &content);
  if (it.ok()) {
    response.set_content(content,
                         palm::http::content_type::APPLICATION_JSON_UTF8);
    response.status = status;
  } else {
    abort(response, it);
  }
}

void mount(httplib::Server& server, const std::string& path,
           std::shared_ptr<grpc::Channel> channel,
           const std::set<GRpcHandler>& handlers);

}  // namespace http
}  // namespace palm

namespace nlohmann {
template <typename T>
struct adl_serializer<boost::optional<T>> {
  static void to_json(json& j, const boost::optional<T>& o) {
    if (o == boost::none) {
      j = nullptr;
    } else {
      j = *o;
    }
  }

  static void from_json(const json& j, boost::optional<T>& o) {
    if (j.is_null()) {
      o = boost::none;
    } else {
      o = j.template get<T>();
    }
  }
};

template <typename T>
struct adl_serializer<std::optional<T>> {
  static void to_json(json& j, const std::optional<T>& o) {
    if (o == std::nullopt) {
      j = nullptr;
    } else {
      j = *o;
    }
  }

  static void from_json(const json& j, std::optional<T>& o) {
    if (j.is_null()) {
      o = std::nullopt;
    } else {
      o = j.template get<T>();
    }
  }
};
}  // namespace nlohmann
