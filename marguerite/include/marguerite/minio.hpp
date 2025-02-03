#pragma once

#include <chrono>
#include <cstdint>
#include <filesystem>
#include <string>
#include <utility>
#include <vector>

#include <miniocpp/client.h>
#include <spdlog/spdlog.h>
#include <inja/inja.hpp>
#include <toml++/toml.hpp>

namespace marguerite {
// https://min.io/docs/minio/linux/developers/javascript/API.html#presignedPutObject
class Minio {
 public:
  Minio(const toml::table& config)
      : Minio(config["host"].value<std::string>().value(),
              config["access-key"].value<std::string>().value(),
              config["secret-key"].value<std::string>().value()) {}
  Minio(const std::string& host, const std::string& access_key,
        const std::string& secret_key)
      : _host(host), _base_url(host), _provider(access_key, secret_key) {}
  bool bucket_exist(const std::string& name);
  std::vector<std::string> list_bucket();
  void remove_bucket(const std::string& name);
  void create_bucket(const std::string& name, bool is_public = false,
                     std::optional<std::chrono::days> expiration_days =
                         std::optional<std::chrono::days>{7});

  std::pair<std::string, uint64_t> put_object(
      const std::string& bucket, const std::filesystem::path& file);
  void remove_object(const std::string& bucket, const std::string& object);

  inline std::string get_permanent_object_url(const std::string& bucket,
                                              const std::string& object) {
    return fmt::format("https://{}/{}/{}", this->_host, bucket, object);
  }
  std::string get_presigned_object_url(
      const std::string& bucket, const std::string& object,
      const std::chrono::seconds& expiry_seconds =
          std::chrono::duration_cast<std::chrono::seconds>(std::chrono::days{
              7}));

 private:
  inline minio::s3::Client open() {
    minio::s3::Client it(this->_base_url, &this->_provider);
    return it;
  }
  std::string object(const std::filesystem::path& file);
  std::string build_bucket_policy(const std::string& name);

  minio::s3::BaseUrl _base_url;
  minio::creds::StaticProvider _provider;
  inja::Environment _env;
  std::string _host;
};
}  // namespace marguerite
