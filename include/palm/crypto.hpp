#pragma once

#include <string>

// openssl rand -base64 32
namespace palm {
std::string uuid();

class Jwt {};
class Hs512 {};
class Aes {};
namespace ssha512 {}
}  // namespace palm
