#pragma once

#include <chrono>
#include <fstream>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

#include <boost/lexical_cast.hpp>
#include <boost/url.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

#include <miniocpp/client.h>
#include <nlohmann/json.hpp>

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace palm {

namespace minio {
struct Config {
  std::string url;
  std::string access_key;
  std::string secret_key;
  std::string api;
  std::string path;

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Config, access_key, secret_key, api, path)
};

class Client {
 public:
  Client(const toml::table& config)
      : _base_url(config["base-url"].value<std::string>().value()),
        _access_key(config["access-key"].value<std::string>().value()),
        _secret_key(config["secret-key"].value<std::string>().value()) {}
  Client(const std::string& base_url, const std::string& access_key,
         const std::string& secret_key)
      : _base_url(base_url), _access_key(access_key), _secret_key(secret_key) {}

  static inline std::shared_ptr<Client> open(const std::string& name) {
    std::ifstream fs(std::format("{}.json", name));
    auto js = nlohmann::json::parse(fs);
    auto cfg = js.template get<Config>();
    const auto url = boost::urls::parse_uri(cfg.url);
    std::stringstream endpoint;
    {
      endpoint << url->scheme() << "://" << url->host();
      if (url->has_port()) {
        endpoint << ":" << url->port();
      }
    }
    auto it = std::make_shared<Client>(endpoint.str(), cfg.access_key,
                                       cfg.secret_key);
    return it;
  }

  std::vector<std::string> list_buckets();
  std::vector<std::tuple<std::string, size_t>> list_objects(
      const std::string& bucket);

  bool bucket_exists(const std::string& name);

  void create_bucket(const std::string& name, bool is_public = false,
                     std::optional<uint> expiration_days = std::nullopt);
  std::optional<std::string> upload(const std::string& bucket,
                                    const std::string& object,
                                    const std::string& file);
  std::optional<std::string> get_presigned_object_url(
      const std::string& bucket, const std::string& object,
      const std::string& title, const std::string& content_type,
      const std::chrono::seconds ttl =
          std::chrono::duration_cast<std::chrono::seconds>(std::chrono::days{
              7}));
  std::string get_object(const std::string& bucket, const std::string& object);
  bool get_object(const std::string& bucket, const std::string& object,
                  std::ofstream& output);
  inline std::string base_url() { return this->_base_url; }

  static inline std::string object(const std::filesystem::path& filename) {
    static boost::uuids::random_generator gen;
    boost::uuids::uuid it = gen();

    std::filesystem::path file(filename);
    return std::format("{}{}", boost::lexical_cast<std::string>(it),
                       file.extension().string());
  }

 private:
  std::string _base_url;
  std::string _access_key;
  std::string _secret_key;
};
}  // namespace minio
}  // namespace palm
