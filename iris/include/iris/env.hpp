#pragma once

#include <filesystem>

#include <toml++/toml.hpp>

namespace iris {
class Storage {
 public:
  virtual void dump(const std::filesystem::path& file) const = 0;
  virtual void restore(const std::filesystem::path& file) const = 0;
};
}  // namespace iris
