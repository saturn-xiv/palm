#pragma once

#include "palm/http.hpp"

#include <cpr/cpr.h>
#include <spdlog/spdlog.h>
#include <toml++/toml.hpp>

namespace palm {

namespace opensearch {
namespace responses {
namespace info {
struct Version {
  std::string distribution;
  std::string number;
  std::string build_type;
  std::string build_hash;
  std::string build_date;
  bool build_snapshot;
  std::string lucene_version;
  std::string minimum_wire_compatibility_version;
  std::string minimum_index_compatibility_version;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Version, distribution, number, build_type,
                                   build_hash, build_date, build_snapshot,
                                   lucene_version,
                                   minimum_wire_compatibility_version,
                                   minimum_index_compatibility_version)
struct Item {
  std::string name;
  std::string cluster_name;
  std::string cluster_uuid;
  std::string tagline;
  Version version;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Item, name, cluster_name, cluster_uuid,
                                   tagline, version)
}  // namespace info

}  // namespace responses
class Config {
 public:
  Config(const toml::table& config)
      : _ssl(config["ssl"].value_or<bool>(false)),
        _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(9200)),
        _namespace(config["namespace"].value_or<std::string>("")) {}
  Config(bool ssl = false, const std::string& host = "127.0.0.1",
         uint16_t port = 9200, const std::string& namespace_ = "")
      : _ssl(ssl), _host(host), _port(port), _namespace(namespace_) {}

  inline std::string url(const std::string& path) const {
    const std::string it =
        std::format("{}://{}:{}/{}", this->_ssl ? "https" : "http", this->_host,
                    this->_port, path);
    return it;
  }

  std::shared_ptr<responses::info::Item> info() const;

  // https://docs.opensearch.org/latest/getting-started/communicate/#indexing-documents
  template <typename T>
  bool index_document(const T& object) const {
    const auto name = this->index<T>();
    spdlog::debug("index document {}", name);
    nlohmann::json body;
    nlohmann::to_json(body, object);

    cpr::Response res = cpr::Put(
        cpr::Url{this->url(name) + "/_doc/"}, cpr::Body{body.dump()},
        cpr::Header{{palm::http::headers::CONTENT_TYPE,
                     palm::http::content_type::APPLICATION_JSON_UTF8}});
    spdlog::debug("{} {}", res.status_code, res.text);
    return res.status_code == 200;
  }
  template <typename T>
  bool index_document(const std::string& id, const T& object) const {
    const auto name = this->index<T>();
    spdlog::debug("index document {}@{}", id, name);
    nlohmann::json body;
    nlohmann::to_json(body, object);

    cpr::Response res = cpr::Put(
        cpr::Url{this->url(name) + "/_doc/" + id}, cpr::Body{body.dump()},
        cpr::Header{{palm::http::headers::CONTENT_TYPE,
                     palm::http::content_type::APPLICATION_JSON_UTF8}});
    spdlog::debug("{} {}", res.status_code, res.text);
    return res.status_code == 200;
  }
  // https://docs.opensearch.org/latest/getting-started/communicate/#updating-documents
  template <typename T>
  bool update_document(const std::string& id,
                       const nlohmann::json& parts) const {
    const auto name = this->index<T>();
    spdlog::debug("update document {}@{}", id, name);
    cpr::Response res = cpr::Post(
        cpr::Url{this->url(name) + "/_update/" + id + "/"},
        cpr::Body{parts.dump()},
        cpr::Header{{palm::http::headers::CONTENT_TYPE,
                     palm::http::content_type::APPLICATION_JSON_UTF8}});
    spdlog::debug("{} {}", res.status_code, res.text);
    return res.status_code == 200;
  }
  // https://docs.opensearch.org/latest/getting-started/communicate/#deleting-a-document
  template <typename T>
  bool delete_document(const std::string& id) const {
    const auto name = this->index<T>();
    spdlog::warn("delete document {}@{}", id, name);
    cpr::Response res = cpr::Delete(cpr::Url{this->url(name) + "/_doc/" + id});
    spdlog::info("{} {}", res.status_code, res.text);
    return res.status_code == 200;
  }

  template <typename T>
  bool close_index() const {
    const auto name = this->index<T>();
    spdlog::warn("close index {}", name);
    cpr::Response res = cpr::Put(cpr::Url{this->url(name) + "/_close"});
    spdlog::info("{} {}", res.status_code, res.text);
    return res.status_code == 200;
  }
  template <typename T>
  bool open_index() const {
    const auto name = this->index<T>();
    spdlog::warn("open index {}", name);
    cpr::Response res = cpr::Post(cpr::Url{this->url(name) + "/_open"});
    spdlog::info("{} {}", res.status_code, res.text);
    return res.status_code == 200;
  }
  // https://docs.opensearch.org/latest/getting-started/communicate/#deleting-an-index
  template <typename T>
  bool delete_index() const {
    const auto name = this->index<T>();
    spdlog::warn("open index {}", name);
    cpr::Response res = cpr::Delete(cpr::Url{this->url(name)});
    spdlog::info("{} {}", res.status_code, res.text);
    return res.status_code == 200;
  }
  template <typename T>
  bool index_exists() const {
    const auto name = this->index<T>();
    spdlog::debug("index exists {}", name);
    cpr::Response res = cpr::Head(cpr::Url{this->url(name)});
    spdlog::debug("{} {}", res.status_code, res.text);
    return res.status_code == 200;
  }

  template <typename T>
  bool create_index(const nlohmann::json& properties) const {
    const auto name = this->index<T>();
    nlohmann::json mappings;
    mappings["properties"] = properties;
    nlohmann::json config;
    config["mappings"] = mappings;

    const std::string body = config.dump();
    spdlog::warn("create index {}: {}", name, body);
    cpr::Response res = cpr::Put(
        cpr::Url{this->url(name)}, cpr::Body{body},
        cpr::Header{{palm::http::headers::CONTENT_TYPE,
                     palm::http::content_type::APPLICATION_JSON_UTF8}});
    if (res.status_code != 200) {
      spdlog::error("{} {}", res.status_code, res.text);
      return false;
    }
    spdlog::info(res.text);
    return true;
  }

 private:
  //  https://docs.opensearch.org/latest/api-reference/index-apis/create-index/#index-naming-restrictions
  template <typename T>
  std::string index() const {
    // const std::string ty = typeid(T).name();
    const std::string ty = boost::typeindex::type_id<T>().pretty_name();
    std::string it = std::format("{}-{}", this->_namespace, ty);
    // std::replace(str.begin(), str.end(), '::', '.');
    // std::transform(it.begin(), it.end(), it.begin(),
    //                [](unsigned char c) { return std::tolower(c); });
    boost::algorithm::replace_all(it, "::", ".");
    boost::algorithm::to_lower(it);
    return it;
  }

  std::string _host;
  uint16_t _port;
  bool _ssl;
  std::string _namespace;
};
}  // namespace opensearch

}  // namespace palm
