#pragma once

#include <filesystem>
#include <format>
#include <string>
#include <vector>

#include <boost/algorithm/string.hpp>

#include <spdlog/spdlog.h>

namespace palm {

void init(bool debug);

inline std::string truncate(const std::string& s, uint l,
                            const std::string& ellipse = "...") {
  int c = l - ellipse.length();
  if (c <= 0 || s.length() <= l) {
    return s;
  }
  return boost::algorithm::erase_tail_copy(s, c) + ellipse;
}

void load(const std::filesystem::path& f, std::string& s);
void load(const std::filesystem::path& f, std::vector<uint8_t> s);

inline bool is_stopped() {
  static const std::string file = ".stop";
  const auto ok = std::filesystem::exists(file);
  if (ok) {
    spdlog::warn("file {} exists, will be exited...", file);
  }
  return ok;
}

namespace gravatar {
std::string hash(const std::string& email);
// https://docs.gravatar.com/api/avatars/images/
inline std::string image(const std::string& email) {
  const auto h = hash(email);
  return std::format("https://gravatar.com/avatar/{}", h);
}
// https://docs.gravatar.com/api/profiles/
inline std::string profile(const std::string& email) {
  const auto h = hash(email);
  return std::format("https://gravatar.com/{}.json", h);
}
}  // namespace gravatar

}  // namespace palm
