#pragma once

#include "palm/pool.hpp"

#include <hiredis/hiredis.h>

namespace palm {

namespace cache {

class Cache {
 public:
  inline void set(const std::string& key, const std::string& val) {
    this->set(key, val, std::nullopt);
  }
  virtual void set(const std::string& key, const std::string& val,
                   const std::optional<std::chrono::seconds> ttl) = 0;
  virtual std::string get(const std::string& key) = 0;
  virtual void heatbeat() = 0;
  virtual void clear() = 0;
  virtual std::vector<std::pair<std::string, int>> keys() = 0;
};

}  // namespace cache
namespace redis {

class Connection : public palm::cache::Cache {
 public:
  Connection(redisContext* context, const std::string& zone)
      : context(context), zone(zone) {}
  ~Connection();

  void set(const std::string& key, const std::string& val,
           const std::optional<std::chrono::seconds> ttl) override;
  std::string get(const std::string& key) override;
  void heatbeat() override;
  void clear() override;
  std::vector<std::pair<std::string, int>> keys() override;

  friend class Config;

 private:
  inline std::string key(const std::string& k) const {
    return this->zone + "://" + k;
  }

  redisContext* context;
  std::string zone;
};

class Config {
 public:
  Config(const std::string& host = "127.0.0.1", uint16_t port = 6379,
         const std::string& zone = "dev", size_t size = 32)
      : host(host), port(port), zone(zone), size(size) {}

  friend std::ostream& operator<<(std::ostream& out, const Config& self) {
    out << "tcp://" << self.host << ":" << self.port << "/" << self.zone;
    return out;
  }

  void operator=(const toml::table& node) {
    {
      auto it = node["host"].value<std::string>();
      if (it) {
        this->host = it.value();
      }
    }
    {
      auto it = node["port"].value<uint16_t>();
      if (it) {
        this->port = it.value();
      }
    }
    {
      auto it = node["zone"].value<std::string>();
      if (it) {
        this->zone = it.value();
      }
    }
    {
      auto it = node["pool-size"].value<uint16_t>();
      if (it) {
        this->size = static_cast<size_t>(it.value());
      }
    }
  }
  friend toml::table& operator<<(toml::table& node, const Config& self) {
    node.insert_or_assign("host", self.host);
    node.insert_or_assign("port", self.port);
    node.insert_or_assign("zone", self.zone);
    node.insert_or_assign("pool-size", static_cast<uint8_t>(self.size));

    return node;
  }

  std::shared_ptr<Connection> build();

  friend class palm::pool::Pool<Connection, Config>;

 private:
  std::string host;
  uint16_t port;
  std::string zone;
  size_t size;
};

using Pool = palm::pool::Pool<Connection, Config>;
using PooledConnection = palm::pool::PooledConnection<Pool, Connection>;

}  // namespace redis

}  // namespace palm
