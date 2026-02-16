#pragma once

#include <google/protobuf/message.h>
#include <spdlog/spdlog.h>
#include <sw/redis++/redis++.h>
#include <toml++/toml.hpp>

namespace palm {
class Redis {
 public:
  Redis(const toml::table& config)
      : _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(6379)),
        _pool_size(config["pool-size"].value_or<size_t>(1 << 5)) {}
  Redis(const std::string& host = "127.0.0.1", uint16_t port = 6379,
        size_t pool_size = 1 << 5)
      : _host(host), _port(port), _pool_size(pool_size) {}
  std::shared_ptr<sw::redis::RedisCluster> open() const;

 private:
  std::string _host;
  uint16_t _port;
  size_t _pool_size;
};

namespace cache {
inline bool set(std::shared_ptr<sw::redis::RedisCluster> client,
                const std::string& key, google::protobuf::Message* value,
                const std::chrono::seconds& ttl = std::chrono::seconds(0)) {
  std::string buf;
  if (!value->SerializeToString(&buf)) {
    spdlog::error("failed to serialize protobuf message");
    return false;
  }
  return client->set(
      key, buf, std::chrono::duration_cast<std::chrono::milliseconds>(ttl));
}
inline bool set(std::shared_ptr<sw::redis::RedisCluster> client,
                const std::string& key, const std::string& value,
                const std::chrono::seconds& ttl = std::chrono::seconds(0)) {
  return client->set(
      key, value, std::chrono::duration_cast<std::chrono::milliseconds>(ttl));
}
inline std::optional<std::string> get(
    std::shared_ptr<sw::redis::RedisCluster> client, const std::string& key) {
  return client->get(key);
}
inline bool get(std::shared_ptr<sw::redis::RedisCluster> client,
                const std::string& key, google::protobuf::Message* value) {
  const auto buf = client->get(key);
  if (!buf.has_value()) {
    spdlog::error("key `{}` didn't exists in cache", key);
    return false;
  }
  return value->ParseFromString(buf.value());
}
}  // namespace cache
}  // namespace palm
