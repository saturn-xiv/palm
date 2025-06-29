#pragma once

#include <sw/redis++/redis++.h>
#include <toml++/toml.hpp>

namespace palm {

namespace redis {
class Node {
 public:
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
