#pragma once

#include "loquat/env.hpp"

#include <cstdint>
#include <optional>
#include <set>
#include <string>
#include <tuple>

namespace loquat {

int init(const std::string& namespace_, bool debug);

namespace aes {
LOQUAT_LIB_API std::string encrypt(const std::string& plain);
LOQUAT_LIB_API std::string decrypt(const std::string& code);
}  // namespace aes

namespace hmac {
LOQUAT_LIB_API std::string sign(const std::string& plain);
LOQUAT_LIB_API void verify(const std::string& code, const std::string& plain);
}  // namespace hmac

namespace jwt {
LOQUAT_LIB_API
std::tuple<std::optional<std::string>, std::optional<std::string>, std::string,
           std::optional<std::string>>
verify(const std::string& token, const std::string& issuer,
       const std::string& audience);
LOQUAT_LIB_API std::string sign(
    const std::string& issuer, const std::string& subject,
    const std::set<std::string> audiences, uint32_t ttl = 5,
    const std::optional<std::string> payload = std::nullopt);
}  // namespace jwt

}  // namespace loquat
