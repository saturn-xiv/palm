#pragma once

#include <string>

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace aloe {
namespace s3 {
void sync(const toml::table& config, const std::string& source,
          const std::string& destination, const std::string& file_list);
void sync(const toml::table& config, const std::string& source,
          const std::string& destination);
void dump(const toml::table& config, const std::string& host);
void restore(const toml::table& config, const std::string& host,
             const std::string& file);
}  // namespace s3
}  // namespace aloe
