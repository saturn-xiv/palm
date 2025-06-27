#pragma once

#include "loquat/env.hpp"

#include <cstdint>
#include <optional>
#include <set>
#include <string>
#include <tuple>

namespace loquat {

int init(bool debug);

namespace aes {
LOQUATLIB_API std::string encrypt(const std::string& plain);
LOQUATLIB_API std::string decrypt(const std::string& code);
}  // namespace aes

namespace hmac {
LOQUATLIB_API std::string sign(const std::string& plain);
LOQUATLIB_API void verify(const std::string& code, const std::string& plain);
}  // namespace hmac

namespace jwt {
LOQUATLIB_API std::tuple<std::optional<std::string>, std::optional<std::string>,
                         std::string, std::optional<std::string>>
verify(const std::string& token, const std::string& issuer,
       const std::string& audience);
LOQUATLIB_API std::string sign(const std::optional<std::string> jwt_id,
                               const std::optional<std::string> key_id,
                               const std::string& issuer,
                               const std::string& subject,
                               const std::set<std::string> audiences,
                               uint32_t ttl,
                               const std::optional<std::string> payload);
}  // namespace jwt

}  // namespace loquat
