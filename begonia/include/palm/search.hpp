#pragma once

#include "palm/http.hpp"
#include "palm/utils.hpp"

#include <algorithm>
#include <cstdint>
#include <format>
#include <string>

#include <cpr/cpr.h>
#include <nlohmann/json.hpp>
#include <toml++/toml.hpp>

namespace palm {
namespace opensearch {
namespace requests {
namespace create_index {
struct SettingsIndex {
  uint16_t number_of_shards;
  uint16_t number_of_replicas;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(SettingsIndex, number_of_shards,
                                 number_of_replicas);
};
struct Settings {
  SettingsIndex index;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Settings, index);
};

struct Mappings {
  nlohmann::json properties;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Mappings, properties);
};

struct Item {
  Settings settings;
  Mappings mappings;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Item, settings, mappings);
};
}  // namespace create_index
}  // namespace requests
namespace responses {
namespace count {
struct Item {
  uint64_t count;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Item, count);
};
}  // namespace count
namespace index_document {
struct Item {
  std::string _index;
  std::string _id;
  int _version;
  std::string result;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Item, _index, _id, _version, result);
};
}  // namespace index_document
namespace create_index {
struct Item {
  bool acknowledged;
  bool shards_acknowledged;
  std::string index;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Item, acknowledged, shards_acknowledged,
                                 index);
};
}  // namespace create_index
namespace delete_index {
struct Item {
  bool acknowledged;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Item, acknowledged);
};
}  // namespace delete_index

namespace create_index {}  // namespace create_index
struct ClusterHealth {
  std::string cluster_name;
  std::string status;
  bool timed_out;
  size_t number_of_nodes;
  size_t number_of_data_nodes;
  bool discovered_master;
  bool discovered_cluster_manager;
  size_t active_primary_shards;
  size_t active_shards;
  size_t relocating_shards;
  size_t initializing_shards;
  size_t unassigned_shards;
  size_t delayed_unassigned_shards;
  size_t number_of_pending_tasks;
  size_t number_of_in_flight_fetch;
  size_t task_max_waiting_in_queue_millis;
  double active_shards_percent_as_number;

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(
      ClusterHealth, cluster_name, status, timed_out, number_of_nodes,
      number_of_data_nodes, discovered_master, discovered_cluster_manager,
      active_primary_shards, active_shards, relocating_shards,
      initializing_shards, unassigned_shards, delayed_unassigned_shards,
      number_of_pending_tasks, number_of_in_flight_fetch,
      task_max_waiting_in_queue_millis, active_shards_percent_as_number)
};
}  // namespace responses
// https://opensearch.org/docs/latest/api-reference/
class Client {
 public:
  Client(const toml::table* config)
      : _host(config->get("host")->value_or<std::string>("127.0.0.1")),
        _port(config->get("port")->value_or<int>(9200)),
        _auth(std::nullopt) {}
  Client(const std::string& host = "127.0.0.1", uint16_t port = 9200,
         std::optional<std::pair<std::string, std::string>> auth = std::nullopt)
      : _host(host), _port(port), _auth(auth) {}

  template <class T>
  std::optional<uint64_t> count() const {
    const auto name = this->index_name<T>();
    const auto res = this->get<responses::count::Item>(name + "/_count");
    if (res) {
      return {res->count};
    }
    return std::nullopt;
    // return res.transform([=](auto x) -> uint { return x.count; });
  }

  template <class T>
  std::optional<responses::index_document::Item> index_document(
      const T& entry) const {
    const auto name = this->index_name<T>();
    return this->post<T, responses::index_document::Item>(name + "/_doc",
                                                          entry);
  }

  template <class T>
  std::optional<responses::create_index::Item> create_index(
      uint16_t number_of_shards, uint16_t number_of_replicas,
      // https://docs.opensearch.org/docs/latest/field-types/
      const nlohmann::json& properties) const {
    const auto name = this->index_name<T>();
    BOOST_LOG_TRIVIAL(warning) << "create index " << name;
    requests::create_index::Item req = {
        .settings = {.index = {.number_of_shards = number_of_shards,
                               .number_of_replicas = number_of_replicas}},
        .mappings = {.properties = properties}};
    return this
        ->put<requests::create_index::Item, responses::create_index::Item>(name,
                                                                           req);
  }
  template <class T>
  bool index_exists() const {
    const auto name = this->index_name<T>();
    const bool ok = this->head(name);
    return ok;
  }
  template <class T>
  void delete_index() const {
    const auto name = this->index_name<T>();
    BOOST_LOG_TRIVIAL(warning) << "delete index " << name;
    this->delete_<responses::delete_index::Item>(name);
  }

  std::optional<responses::ClusterHealth> cluster_health() const {
    return this->get<responses::ClusterHealth>("_cluster/health");
  }

  template <class T>
  std::string index_name() const {
    std::string it = boost::typeindex::type_id<T>().pretty_name();
    // std::replace(it.begin(), it.end(), ':', '.');
    boost::replace_all(it, "::", ".");
    boost::algorithm::to_lower(it);
    return it;
  }

 private:
  template <class R>
  std::optional<R> get(const std::string& path) const {
    const auto url = this->url(path);
    BOOST_LOG_TRIVIAL(debug) << "GET " << url;
    cpr::Response res = cpr::Get(cpr::Url{url});
    if (res.status_code != palm::http::status::OK) {
      BOOST_LOG_TRIVIAL(error) << res.status_code << " " << res.text;
      return std::nullopt;
    }
    BOOST_LOG_TRIVIAL(debug) << palm::truncate(res.text, MESSAGE_SIZE);
    auto js = nlohmann::json::parse(res.text);
    return js.template get<R>();
  }
  template <class Q, class R>
  std::optional<R> post(const std::string& path, const Q& request) const {
    nlohmann::json body = request;

    const auto url = this->url(path);
    BOOST_LOG_TRIVIAL(debug) << "POST " << url << "\n"
                             << palm::truncate(body.dump(2), MESSAGE_SIZE);
    cpr::Response res = cpr::Post(
        cpr::Url{url}, cpr::Body{body.dump()},
        cpr::Header{{palm::http::headers::CONTENT_TYPE,
                     palm::http::content_type::APPLICATION_JSON_UTF8}});
    if (res.status_code != palm::http::status::OK &&
        res.status_code != palm::http::status::CREATED) {
      BOOST_LOG_TRIVIAL(error) << res.status_code << " " << res.text;
      return std::nullopt;
    }
    BOOST_LOG_TRIVIAL(debug) << palm::truncate(res.text, MESSAGE_SIZE);
    auto js = nlohmann::json::parse(res.text);
    return js.template get<R>();
  }
  template <class R>
  std::optional<R> put(const std::string& path) const {
    const auto url = this->url(path);
    BOOST_LOG_TRIVIAL(debug) << "PUT " << url;
    cpr::Response res = cpr::Put(cpr::Url{url});
    if (res.status_code != palm::http::status::OK) {
      BOOST_LOG_TRIVIAL(error) << res.status_code << " " << res.text;
      return std::nullopt;
    }
    BOOST_LOG_TRIVIAL(debug) << palm::truncate(res.text, MESSAGE_SIZE);
    auto js = nlohmann::json::parse(res.text);
    return js.template get<R>();
  }
  template <class Q, class R>
  std::optional<R> put(const std::string& path, const Q& request) const {
    nlohmann::json body = request;

    const auto url = this->url(path);
    BOOST_LOG_TRIVIAL(debug) << "PUT " << url << "\n"
                             << palm::truncate(body.dump(2), MESSAGE_SIZE);
    cpr::Response res = cpr::Put(
        cpr::Url{url}, cpr::Body{body.dump()},
        cpr::Header{{palm::http::headers::CONTENT_TYPE,
                     palm::http::content_type::APPLICATION_JSON_UTF8}});
    if (res.status_code != palm::http::status::OK) {
      BOOST_LOG_TRIVIAL(error) << res.status_code << " " << res.text;
      return std::nullopt;
    }
    BOOST_LOG_TRIVIAL(debug) << palm::truncate(res.text, MESSAGE_SIZE);
    auto js = nlohmann::json::parse(res.text);
    return js.template get<R>();
  }
  bool head(const std::string& path) const {
    const auto url = this->url(path);
    BOOST_LOG_TRIVIAL(debug) << "HEAD " << url;
    cpr::Response res = cpr::Head(cpr::Url{url});
    if (res.status_code != palm::http::status::OK) {
      BOOST_LOG_TRIVIAL(error) << res.status_code << " " << res.text;
      return false;
    }
    BOOST_LOG_TRIVIAL(debug) << palm::truncate(res.text, MESSAGE_SIZE);
    return true;
  }
  template <class R>
  std::optional<R> head(const std::string& path) const {
    const auto url = this->url(path);
    BOOST_LOG_TRIVIAL(debug) << "HEAD " << url;
    cpr::Response res = cpr::Head(cpr::Url{url});
    if (res.status_code != palm::http::status::OK) {
      BOOST_LOG_TRIVIAL(error) << res.status_code << " " << res.text;
      return std::nullopt;
    }
    BOOST_LOG_TRIVIAL(debug) << palm::truncate(res.text, MESSAGE_SIZE);
    auto js = nlohmann::json::parse(res.text);
    return js.template get<R>();
  }
  template <class R>
  std::optional<R> delete_(const std::string& path) const {
    const auto url = this->url(path);
    BOOST_LOG_TRIVIAL(debug) << "DELETE " << url;
    cpr::Response res = cpr::Delete(cpr::Url{url});
    if (res.status_code != palm::http::status::OK) {
      BOOST_LOG_TRIVIAL(error) << res.status_code << " " << res.text;
      return std::nullopt;
    }
    BOOST_LOG_TRIVIAL(debug) << palm::truncate(res.text, MESSAGE_SIZE);
    auto js = nlohmann::json::parse(res.text);
    return js.template get<R>();
  }

  inline std::string url(const std::string& path) const {
    return std::format("http://{}:{}/{}", this->_host, this->_port, path);
  }

  static const uint MESSAGE_SIZE = 256;

  std::string _host;
  uint16_t _port;
  std::optional<std::pair<std::string, std::string>> _auth;
};
}  // namespace opensearch

}  // namespace palm
