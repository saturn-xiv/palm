#pragma once

#include "lavender/open-search.hpp"

#include <filesystem>

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

class File {
 public:
  File(const std::filesystem::path& name);
  ~File();

  std::string read();

 private:
  FILE* _file;
  std::mutex _mutex;
};

class Watcher {
 public:
  Watcher(std::shared_ptr<lavender::OpenSearch> search,
          const std::filesystem::path& path);
  ~Watcher();

  void watch();

 private:
  std::shared_ptr<lavender::OpenSearch> _search;
  std::mutex _mutex;
  std::map<std::filesystem::path, std::shared_ptr<File>> _items;
  std::filesystem::path _root;

  int _file;
  int _watcher;
};

}  // namespace filesystem
}  // namespace logging
}  // namespace lavender
