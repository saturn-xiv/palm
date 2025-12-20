#pragma once

#include "iris/env.hpp"

namespace iris {
class Filesystem : public Storage {
 public:
  Filesystem(const toml::table& config)
      : _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(22)),
        _user(config["user"].value_or<std::string>("root")),
        _key_file(config["key-file"].value<std::string>()),
        _excludes({}),
        _folder(config["folder"].value<std::string>().value()) {
    const toml::array* items = config["excludes"].as_array();
    if (items != nullptr) {
      for (const auto& it : *items) {
        this->_excludes.push_back(it.value<std::string>().value());
      }
    }
  }

  void dump(const std::filesystem::path& folder) const override;
  void restore(const std::filesystem::path& file) const override;
  void upload(const std::filesystem::path& file) const;

 private:
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::optional<std::string> _key_file;
  std::string _folder;
  std::vector<std::string> _excludes;

  std::filesystem::path key_file() const;
};

}  // namespace iris
