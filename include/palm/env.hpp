#pragma once

#include <filesystem>
#include <fstream>
#include <ostream>
#include <string>

namespace palm {
inline std::string file2str(const std::filesystem::path& file) {
  std::ifstream fs;
  std::ios_base::iostate mask = fs.exceptions() | std::ios::failbit;
  fs.exceptions(mask);
  fs.open(file);
  return std::string((std::istreambuf_iterator<char>(fs)),
                     std::istreambuf_iterator<char>());
}
}  // namespace palm
