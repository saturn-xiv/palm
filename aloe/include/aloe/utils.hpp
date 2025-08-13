#pragma once

#include <filesystem>

namespace aloe {
void keep(const std::filesystem::path& target, const size_t count);
inline std::string filename(const std::filesystem::path& file) {
  auto it = file.stem();
  for (;;) {
    if (!it.has_extension()) {
      return it.stem().string();
    }
    it = file.stem();
  }
}
}  // namespace aloe
