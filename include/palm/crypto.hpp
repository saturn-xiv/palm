#pragma once

#include <string>
#include <vector>

// openssl rand -base64 32
namespace palm {
std::string uuid();

namespace random {
std::vector<uint8_t> bytes(const size_t len);
std::string string(const size_t len);
}  // namespace random
namespace base64 {
std::string to(const std::vector<uint8_t>& buf);
std::vector<uint8_t> from(const std::string& str);
}  // namespace base64

class Jwt {};
class Hs512 {};
class Aes {};
namespace ssha512 {}
}  // namespace palm
