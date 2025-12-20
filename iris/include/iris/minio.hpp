#pragma once

#include "iris/env.hpp"

namespace iris {

class Minio : public Storage {
 public:
  Minio(const toml::table& config)
      : _alias(config["alias"].value_or<std::string>("default")),
        _url(config["url"].value<std::string>().value()),
        _access_key(config["access-key"].value<std::string>().value()),
        _secret_key(config["secret-key"].value<std::string>().value()) {}

  void dump(const std::filesystem::path& output) const override;
  void restore(const std::filesystem::path& file) const override;

 private:
  std::string _alias;
  std::string _url;
  std::string _access_key;
  std::string _secret_key;

  void set_alias() const;
};
}  // namespace iris
