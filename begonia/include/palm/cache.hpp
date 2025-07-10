#pragma once

#include <sw/redis++/redis++.h>

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace palm {

namespace redis {
class Node {
 public:
  Node(const toml::table& config)
      : _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(6379)),
        _password(config["password"].value<std::string>()),
        _db(config["db"].value_or<uint8_t>(0)),
        _pool_size(config["pool-size"].value_or<size_t>(1 << 5)) {}
  Node(const std::string& host = "127.0.0.1", uint16_t port = 6379,
       std::optional<std::string> password = std::nullopt, uint8_t db = 0,
       size_t pool_size = 1 << 5)
      : _host(host),
        _port(port),
        _password(password),
        _db(db),
        _pool_size(pool_size) {}
  std::shared_ptr<sw::redis::Redis> open() const;

 private:
  std::string _host;
  uint16_t _port;
  std::optional<std::string> _password;
  uint8_t _db;
  size_t _pool_size;
};
}  // namespace redis

}  // namespace palm
