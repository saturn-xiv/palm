#pragma once

#include <cstdint>
#include <string>
#include <vector>

namespace palm {
namespace captcha {
std::vector<uint8_t> png(const std::string& str, uint8_t size);
}
}  // namespace palm
