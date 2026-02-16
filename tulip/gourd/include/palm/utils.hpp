#pragma once

#include <spdlog/spdlog.h>
#include <toml++/toml.hpp>

namespace palm {
std::string uuid();

namespace base64 {
std::vector<uint8_t> from(const std::string& str);
std::string to(const std::vector<uint8_t>& buf);
}  // namespace base64

namespace random {
std::vector<uint8_t> bytes(size_t len);
std::string alphanumeric(size_t len);
}  // namespace random
}  // namespace palm
