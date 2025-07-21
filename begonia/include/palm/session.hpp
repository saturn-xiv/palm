#pragma once

#include "palm/jwt.hpp"

#include <boost/algorithm/string.hpp>

#include <grpcpp/grpcpp.h>
#include <httplib.h>

namespace palm {
class Session {
 public:
  Session(grpc::ServerContext* context) : _token(Session::token(context)) {}
  Session(const httplib::Request& request)
      : _token(Session::token(request)),
        _locale(Session::locale(request)),
        _client_ip(Session::client_ip(request)) {}

  // https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md
  static inline void init(const httplib::Request& request,
                          grpc::ClientContext* context) {
    if (request.has_header(HTTP_AUTHORIZATION)) {
      const auto it = request.get_header_value(HTTP_AUTHORIZATION);
      context->AddMetadata(GRPC_AUTHORIZATION, it);
    }
  }

  //   https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md
  //   https://jwt.io/introduction
  inline static const std::string GRPC_AUTHORIZATION = "authorization";
  inline static const std::string HTTP_AUTHORIZATION = "Authorization";
  inline static const std::string HTTP_ACCEPT_LANGUAGE = "Accept-Language";
  inline static const std::string BEARER = "Bearer ";
  inline static const std::string LOCALE = "locale";

 protected:
  static inline std::optional<std::string> token(grpc::ServerContext* context) {
    const auto metadata = context->client_metadata();

    auto items = metadata | std::views::filter([](auto& v) {
                   return v.first == GRPC_AUTHORIZATION;
                 });

    for (auto it : items) {
      if (it.second.starts_with(BEARER)) {
        const std::string s(it.second.begin(), it.second.end());
        return s.substr(BEARER.size());
      }
    }

    return std::nullopt;
  }

  static inline std::optional<std::string> token(
      const httplib::Request& request) {
    if (request.has_header(HTTP_AUTHORIZATION)) {
      const auto it = request.get_header_value(HTTP_AUTHORIZATION);
      if (it.starts_with(BEARER)) {
        return it.substr(BEARER.size());
      }
    }
    return std::nullopt;
  }

  // https://github.com/svenfuchs/rails-i18n?tab=readme-ov-file#available-locales-1
  // https://guides.rubyonrails.org/i18n.html#choosing-an-implied-locale
  static inline std::optional<std::string> locale(
      const httplib::Request& request) {
    if (request.has_param(LOCALE)) {
      return request.get_param_value(LOCALE);
    }
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Accept-Language
    if (request.has_header(HTTP_ACCEPT_LANGUAGE)) {
      const auto it = request.get_header_value(HTTP_ACCEPT_LANGUAGE);
      std::vector<std::string> items;
      boost::split(items, it, boost::is_any_of(","));
      if (!items.empty()) {
        return items[0];
      }
    }
    return std::nullopt;
  }
  // https://pkg.go.dev/github.com/gin-gonic/gin#Context.ClientIP
  static inline std::optional<std::string> client_ip(
      const httplib::Request& request) {
    // TODO
    return std::nullopt;
  }
  std::optional<std::string> _token;
  std::optional<std::string> _locale;
  std::optional<std::string> _client_ip;
};
}  // namespace palm
