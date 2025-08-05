#include "aloe/utils.hpp"

#include <vector>

#include <spdlog/spdlog.h>

void aloe::keep(const std::filesystem::path& target, const size_t count) {
  // TODO
  std::vector<std::filesystem::path> items;
  for (const auto& it : std::filesystem::directory_iterator(target)) {
    const auto file = it.path();
    if (std::filesystem::is_regular_file(file)) {
      spdlog::debug("find file {}", file.string());
      items.push_back(file);
    }
  }
  std::sort(items.begin(), items.end(),
            [](const auto& a, const auto& b) -> bool {
              return std::filesystem::last_write_time(a) >
                     std::filesystem::last_write_time(b);
            });

  spdlog::info("find {} backup files", items.size());
  if (items.size() > count) {
    for (auto it = items.begin() + count; it != items.end(); ++it) {
      spdlog::warn("remove file {}", it->string());
      std::filesystem::remove(*it);
    }
  }
}
