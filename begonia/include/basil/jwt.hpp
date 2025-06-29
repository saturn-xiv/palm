#pragma once

#include "basil/http.hpp"

#include <chrono>
#include <optional>
#include <ranges>
#include <set>
#include <vector>

#include <boost/algorithm/string.hpp>

#include <grpcpp/grpcpp.h>
#include <httplib.h>

namespace basil {
class Jwt {
 public:
  Jwt(const std::string& key) : _key(key) {}

  inline std::string sign(const std::string& issuer, const std::string& subject,
                          const std::set<std::string> audiences,
                          const std::optional<std::string> payload,
                          const std::chrono::seconds ttl =
                              std::chrono::duration_cast<std::chrono::seconds>(
                                  std::chrono::days{7})) const {
    const auto now = std::chrono::system_clock::now();
    const auto exp = now + ttl;
    const auto nbf = now + std::chrono::seconds{-1};
    return this->sign(std::nullopt, std::nullopt, issuer, subject, audiences,
                      now, nbf, exp, payload);
  }

  std::string sign(
      const std::optional<std::string> jwt_id,
      const std::optional<std::string> key_id, const std::string& issuer,
      const std::string& subject, const std::set<std::string> audiences,
      const std::chrono::time_point<std::chrono::system_clock>& issued_at,
      const std::chrono::time_point<std::chrono::system_clock>& not_before,
      const std::chrono::time_point<std::chrono::system_clock>& expired_at,
      const std::optional<std::string> payload) const;
  /*
  jwt_id, key_id, subject, payload
  */
  std::tuple<std::optional<std::string>, std::optional<std::string>,
             std::string, std::optional<std::string>>
  verify(const std::string& token, const std::string& issuer,
         const std::string& audience) const;

  static inline std::optional<std::string> token(grpc::ServerContext* context) {
    static const std::string auth =
        boost::algorithm::to_lower_copy(basil::http::headers::AUTHORIZATION);
    const auto metadata = context->client_metadata();

    auto items =
        metadata | std::views::filter([](auto& v) { return v.first == auth; });

    for (auto it : items) {
      if (it.second.starts_with(basil::http::headers::BEARER)) {
        const std::string s(it.second.begin(), it.second.end());
        return s.substr(basil::http::headers::BEARER.size());
      }
    }

    return std::nullopt;
  }

  static inline std::optional<std::string> token(
      const httplib::Request& request) {
    if (request.has_header(basil::http::headers::AUTHORIZATION)) {
      const auto it =
          request.get_header_value(basil::http::headers::AUTHORIZATION);
      if (it.starts_with(basil::http::headers::BEARER)) {
        return it.substr(basil::http::headers::BEARER.size());
      }
    }
    return std::nullopt;
  }

 private:
  std::string _key;
  inline static const std::string PAYLOAD_CLAIM_KEY = "ext";
};
}  // namespace basil
