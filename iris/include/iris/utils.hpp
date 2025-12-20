#pragma once

#include <filesystem>
#include <initializer_list>
#include <tuple>

#include <inja/inja.hpp>
#include <nlohmann/json.hpp>

namespace iris {
std::string timestamp(const std::string &prefix);
void keep(const std::filesystem::path &folder, const std::string &prefix,
          size_t count);
void uncompress(const std::filesystem::path &folder,
                const std::filesystem::path &tar);
void compress(const std::filesystem::path &folder);
int md5(const std::filesystem::path &file);
std::tuple<std::string, std::string, int> execute(
    const std::vector<std::string> args);
inline std::tuple<std::string, std::string, int> execute(
    const std::initializer_list<std::string> args) {
  std::vector<std::string> items(args);
  return execute(items);
}
// inline std::tuple<std::string, std::string, int> execute(
//     const std::string &args...) {
//   std::vector<std::string> items;
//   // items.reserve(sizeof...(args));
//   (items.push_back(std::forward<std::string>(args)...));
//   // std::vector<std::string> items({args...});
//   return execute(items);
// }
void check(const std::tuple<std::string, std::string, int> res);
std::string uuid();
std::filesystem::path home();
bool is_alphanumeric(const std::string &s);
}  // namespace iris
