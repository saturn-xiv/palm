#pragma once

#include <chrono>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include <miniocpp/client.h>

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace palm {

class Minio {
 public:
  Minio(const toml::table& config)
      : _base_url(config["base-url"].value<std::string>().value()),
        _access_key(config["access-key"].value<std::string>().value()),
        _secret_key(config["secret-key"].value<std::string>().value()) {}
  Minio(const std::string& base_url, const std::string& access_key,
        const std::string& secret_key)
      : _base_url(base_url), _access_key(access_key), _secret_key(secret_key) {}

  std::vector<std::string> list_buckets();
  bool bucket_exists(const std::string& name);

  void create_bucket(const std::string& name, bool is_public,
                     std::optional<uint> expiration_days = std::nullopt);
  std::optional<std::string> upload(const std::string& bucket,
                                    const std::string& file);
  std::optional<std::string> get_presigned_object_url(
      const std::string& bucket, const std::string& object,
      const std::string& title, const std::string& content_type,
      const std::chrono::seconds ttl =
          std::chrono::duration_cast<std::chrono::seconds>(std::chrono::days{
              7}));
  std::string get_object(const std::string& bucket, const std::string& object);

 private:
  std::string _base_url;
  std::string _access_key;
  std::string _secret_key;
};

}  // namespace palm
