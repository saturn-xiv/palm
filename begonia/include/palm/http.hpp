#pragma once

#include <format>
#include <string>

namespace palm {
namespace http {
namespace headers {
inline static const std::string CONTENT_TYPE = "Content-Type";

//   https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md
//   https://jwt.io/introduction
inline static const std::string AUTHORIZATION = "Authorization";
inline static const std::string BEARER = "Bearer ";
}  // namespace headers
namespace content_type {
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
}  // namespace http
}  // namespace palm
