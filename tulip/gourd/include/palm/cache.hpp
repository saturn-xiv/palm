#pragma once

#include <format>

#include <google/protobuf/message.h>
#include <spdlog/spdlog.h>
#include <sw/redis++/redis++.h>
#include <toml++/toml.hpp>

namespace palm {

namespace redis {

class Client {
 public:
  Client(std::unique_ptr<sw::redis::RedisCluster> pool,
         const std::string& namespace_)
      : _pool(std::move(pool)), _namespace(namespace_) {}

  inline bool set(const std::string& key, google::protobuf::Message* value,
                  const std::chrono::seconds& ttl = std::chrono::seconds(0)) {
    std::string buf;
    if (!value->SerializeToString(&buf)) {
      spdlog::error("failed to serialize protobuf message");
      return false;
    }
    return this->_pool->set(
        this->key(key), buf,
        std::chrono::duration_cast<std::chrono::milliseconds>(ttl));
  }
  inline bool set(const std::string& key, const std::string& value,
                  const std::chrono::seconds& ttl = std::chrono::seconds(0)) {
    return this->_pool->set(
        this->key(key), value,
        std::chrono::duration_cast<std::chrono::milliseconds>(ttl));
  }
  inline std::optional<std::string> get(const std::string& key) {
    return this->_pool->get(this->key(key));
  }
  inline bool get(const std::string& key, google::protobuf::Message* value) {
    const auto buf = this->_pool->get(this->key(key));
    if (!buf.has_value()) {
      spdlog::error("couldn't found record {}@{} in cache", key,
                    this->_namespace);
      return false;
    }
    return value->ParseFromString(buf.value());
  }

 private:
  inline std::string key(const std::string& s) const {
    std::string it = std::format("{}://{}", this->_namespace, s);
    return it;
  }

  std::unique_ptr<sw::redis::RedisCluster> _pool;
  std::string _namespace;
};

class Config {
 public:
  Config(const toml::table& config)
      : _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(6379)),
        _namespace(config["namespace"].value_or<std::string>("")),
        _pool_size(config["pool-size"].value_or<size_t>(1 << 5)) {}
  Config(const std::string& host = "127.0.0.1", uint16_t port = 6379,
         const std::string& namespace_ = "", size_t pool_size = 1 << 5)
      : _host(host),
        _port(port),
        _namespace(namespace_),
        _pool_size(pool_size) {}
  std::shared_ptr<Client> open() const;

 private:
  std::string _host;
  uint16_t _port;
  size_t _pool_size;
  std::string _namespace;
};
}  // namespace redis

}  // namespace palm
