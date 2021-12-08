#pragma once

#include <ctime>
#include <filesystem>
#include <optional>
#include <string>
#include <vector>

namespace palm {
namespace orm {
class Migration {
 public:
  Migration(const std::filesystem::path& root);

 private:
  std::string up;
  std::string down;
  std::string version;
  std::string name;
  std::optional<std::tm> run_at;
  std::tm created_at;
};
class Schema {
 public:
  Schema(const std::filesystem::path& root);
  void migrate();
  void rollback();

 private:
  std::vector<Migration> migrations;
};
}  // namespace orm
}  // namespace palm
