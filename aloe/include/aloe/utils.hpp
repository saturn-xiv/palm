#pragma once

#include <cstdint>
#include <filesystem>

namespace aloe {
bool tcp(const std::string& host, uint16_t port);
void keep(const std::filesystem::path& target, const size_t count);
inline std::string filename(const std::filesystem::path& file) {
  auto it = file.stem();
  for (;;) {
    if (!it.has_extension()) {
      return it.stem().string();
    }
    it = it.stem();
  }
}
}  // namespace aloe
