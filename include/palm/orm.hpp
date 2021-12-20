#pragma once

#include <chrono>
#include <cstdint>
#include <ctime>
#include <filesystem>
#include <mutex>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

#include <boost/property_tree/ptree.hpp>

namespace palm {
namespace orm {
class Query {
 public:
  Query(Query const&) = delete;
  void operator=(Query const&) = delete;

  static Query& instance() {
    static Query it;
    return it;
  }
  inline std::optional<std::string> get(const std::string& key) const {
    for (const auto& [_, tree] : this->trees) {
      const auto it = tree.get_optional<std::string>(key);
      if (it) {
        return it.value();
      }
    }
    return std::nullopt;
  }

  void load(const std::filesystem::path& file);

 private:
  Query() {}
  std::mutex locker;
  std::unordered_map<std::string, boost::property_tree::ptree> trees;
};

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
  void status();

 private:
  std::vector<Migration> migrations;
};
}  // namespace orm
}  // namespace palm
