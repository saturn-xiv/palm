#pragma once

// https://developer.ibm.com/tutorials/l-ubuntu-inotify/

#include <nlohmann/json.hpp>

namespace lavender {
namespace logging {
namespace filesystem {
struct Message {
  std::string host;
  std::string file;
  std::string line;
  std::chrono::time_point<std::chrono::high_resolution_clock> created_at;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Message, file, line, created_at)
}  // namespace filesystem
}  // namespace logging
}  // namespace lavender
