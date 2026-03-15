#pragma once

#include <format>

#include <google/protobuf/message.h>
#include <google/protobuf/util/json_util.h>
#include <google/protobuf/util/time_util.h>
#include <httplib.h>
#include <spdlog/spdlog.h>
#include <boost/algorithm/string.hpp>
#include <boost/beast/http.hpp>
#include <boost/beast/version.hpp>
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

// https://www.iana.org/assignments/media-types/media-types.xhtml
inline std::string detect(const std::filesystem::path& path) {
  const auto ext = path.extension();
  if (ext == ".png") {
    return IMAGE_PNG;
  }
  if (ext == ".css") {
    return "text/css";
  }
  if (ext == ".js") {
    return "text/javascript; charset=UTF-8";
  }
  return "application/octet-stream";
}

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

inline boost::beast::http::message_generator response(
    const boost::beast::http::request<boost::beast::http::string_body>& req,
    boost::beast::http::status status, const std::string& content_type,
    const std::string& body = "") {
  boost::beast::http::response<boost::beast::http::string_body> res{
      status, req.version()};
  res.set(boost::beast::http::field::server, BOOST_BEAST_VERSION_STRING);
  res.set(boost::beast::http::field::content_type, content_type);
  res.keep_alive(req.keep_alive());
  res.body() = body;
  res.prepare_payload();
  spdlog::info("{} {} bytes", res.result_int(), res.payload_size().value_or(0));
  return res;
}

inline boost::beast::http::message_generator text(
    const boost::beast::http::request<boost::beast::http::string_body>& req,
    boost::beast::http::status status, const std::string& body = "") {
  return response(req, status, content_type::TEXT_PLAIN_UTF8, body);
}

inline boost::beast::http::message_generator bad_request(
    const boost::beast::http::request<boost::beast::http::string_body>& req,
    const std::string& body = "") {
  return text(req, boost::beast::http::status::bad_request, body);
}
inline boost::beast::http::message_generator not_found(
    const boost::beast::http::request<boost::beast::http::string_body>& req,
    const std::string& body = "") {
  return text(req, boost::beast::http::status::not_found, body);
}
inline boost::beast::http::message_generator internal_server_error(
    const boost::beast::http::request<boost::beast::http::string_body>& req,
    const std::string& body = "") {
  return text(req, boost::beast::http::status::internal_server_error, body);
}
inline boost::beast::http::message_generator forbidden(
    const boost::beast::http::request<boost::beast::http::string_body>& req,
    const std::string& body = "") {
  return text(req, boost::beast::http::status::forbidden, body);
}

inline boost::beast::http::message_generator json(
    const boost::beast::http::request<boost::beast::http::string_body>& req,
    const google::protobuf::Message& data) {
  std::string buf;
  const auto status = google::protobuf::util::MessageToJsonString(data, &buf);
  if (status.ok()) {
    return response(req, boost::beast::http::status::ok,
                    content_type::APPLICATION_JSON_UTF8, buf);
  } else {
    return internal_server_error(
        req, R"(serialize protobuf message to json failed)");
  }
}
inline boost::beast::http::message_generator html(
    const boost::beast::http::request<boost::beast::http::string_body>& req,
    std::shared_ptr<inja::Environment> env, const std::string& tpl,
    const google::protobuf::Message& data) {
  std::string buf;
  {
    const auto status = google::protobuf::util::MessageToJsonString(data, &buf);
    if (!status.ok()) {
      return internal_server_error(
          req, R"(serialize protobuf message to json failed)");
    }
  }

  const nlohmann::json arg = nlohmann::json::parse(buf);
  const std::string body = env->render_file(tpl, arg);
  return response(req, boost::beast::http::status::ok,
                  content_type::TEXT_HTML_UTF8, body);
}

inline boost::beast::http::message_generator file(
    const boost::beast::http::request<boost::beast::http::string_body>& req,
    const std::filesystem::path& path) {
  if (!std::filesystem::exists(path)) {
    return not_found(req, path.filename().c_str());
  }
  boost::beast::http::file_body::value_type it;
  {
    boost::system::error_code ec;
    it.open(path.c_str(), boost::beast::file_mode::scan, ec);
    if (ec) {
      return internal_server_error(req, ec.message());
    }
  }

  boost::beast::http::response<boost::beast::http::file_body> res{
      boost::beast::http::status::ok, req.version()};
  res.set(boost::beast::http::field::server, BOOST_BEAST_VERSION_STRING);
  res.set(boost::beast::http::field::content_type, content_type::detect(path));
  res.body() = std::move(it);
  res.prepare_payload();
  spdlog::info("{} {} bytes", res.result_int(), res.payload_size().value_or(0));
  return res;
}

}  // namespace http

}  // namespace palm

namespace nlohmann {
template <typename Clock, typename Duration>
struct adl_serializer<std::chrono::time_point<Clock, Duration>> {
  static void to_json(nlohmann::json& j,
                      const std::chrono::time_point<Clock, Duration>& o) {
    j = std::format("{:%FT%T%z}", o);
  }

  static void from_json(const nlohmann::json& j,
                        std::chrono::time_point<Clock, Duration>& o) {
    const std::string s = j.get<std::string>();
    std::istringstream in{s};
    in >> std::chrono::parse("%FT%T%z", o);
    if (in.fail()) {
      spdlog::error("failed to parse {}", s);
    }
  }
};
}  // namespace nlohmann
