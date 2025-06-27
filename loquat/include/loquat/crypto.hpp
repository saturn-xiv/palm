#pragma once

#include <cstdint>
#include <optional>
#include <set>
#include <string>
#include <tuple>

namespace loquat {

int init(bool debug);

namespace aes {
std::string encrypt(const std::string& plain);
std::string decrypt(const std::string& code);
}  // namespace aes

namespace hmac {
std::string sign(const std::string& plain);
void verify(const std::string& code, const std::string& plain);
}  // namespace hmac

namespace jwt {
std::tuple<std::optional<std::string>, std::optional<std::string>, std::string,
           std::optional<std::string>>
verify(const std::string& token, const std::string& issuer,
       const std::string& audience);
std::string sign(const std::optional<std::string> jwt_id,
                 const std::optional<std::string> key_id,
                 const std::string& issuer, const std::string& subject,
                 const std::set<std::string> audiences, uint32_t ttl,
                 const std::optional<std::string> payload);
}  // namespace jwt

}  // namespace loquat
