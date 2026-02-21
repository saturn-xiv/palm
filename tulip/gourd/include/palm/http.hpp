#pragma once

#include <format>

#include <google/protobuf/message.h>
#include <google/protobuf/util/json_util.h>
#include <google/protobuf/util/time_util.h>
#include <httplib.h>
#include <boost/algorithm/string.hpp>
#include <boost/type_index.hpp>
#include <inja/inja.hpp>

namespace palm {
namespace http {
namespace headers {
inline static const std::string CONTENT_TYPE = "Content-Type";

}  // namespace headers
namespace content_type {
inline static const std::string IMAGE_PNG = "image/png";
inline static const std::string TEXT_HTML_UTF8 = "text/html; charset=utf-8";
inline static const std::string TEXT_PLAIN_UTF8 = "text/plain; charset=utf-8";
inline static const std::string APPLICATION_JSON_UTF8 =
    "application/json; charset=utf-8";
// https://www.rfc-editor.org/rfc/rfc3023
inline static const std::string APPLICATION_XML = "application/xml";
// https://developers.cloudflare.com/speed/optimization/content/compression/
inline static const std::string APPLICATION_X_PROTOBUF =
    "application/x-protobuf";

}  // namespace content_type
inline static const std::string XML_HEADER_UTF8 =
    R"(<?xml version="1.0" encoding="UTF-8"?>)";

inline void text(httplib::Response& res, httplib::StatusCode status,
                 const std::string& body = "") {
  res.set_content(body, content_type::TEXT_PLAIN_UTF8);
  res.status = status;
}

inline void internal_server_error(httplib::Response& res,
                                  const std::string& body = "") {
  text(res, httplib::StatusCode::InternalServerError_500, body);
}
inline void bad_request(httplib::Response& res, const std::string& body = "") {
  text(res, httplib::StatusCode::BadRequest_400, body);
}

inline bool body(const httplib::Request& req, httplib::Response& res,
                 google::protobuf::Message* model) {
  {
    const auto status =
        google::protobuf::util::JsonStringToMessage(req.body, model);
    if (status.ok()) {
      return true;
    }
    bad_request(res);
    return false;
  }
}

inline void json(httplib::Response& res,
                 const google::protobuf::Message& data) {
  std::string buf;
  const auto status = google::protobuf::util::MessageToJsonString(data, &buf);
  if (status.ok()) {
    res.set_content(buf, content_type::APPLICATION_JSON_UTF8);
    res.status = httplib::StatusCode::OK_200;
  } else {
    internal_server_error(res, "serialize to json failed");
  }
}
inline void html(httplib::Response& res, std::shared_ptr<inja::Environment> env,
                 const std::string& tpl,
                 const google::protobuf::Message& data) {
  std::string buf;
  {
    const auto status = google::protobuf::util::MessageToJsonString(data, &buf);
    if (!status.ok()) {
      internal_server_error(res, "serialize to json failed");
      return;
    }
  }

  const nlohmann::json arg = nlohmann::json::parse(buf);
  const std::string body = env->render_file(tpl, arg);
  res.set_content(body, content_type::TEXT_HTML_UTF8);
  res.status = httplib::StatusCode::OK_200;
}

}  // namespace http

}  // namespace palm
