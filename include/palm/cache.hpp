#pragma once

#include <chrono>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <optional>
#include <ostream>
#include <string>
#include <utility>
#include <vector>

#include <boost/property_tree/ptree.hpp>

#include <Poco/ObjectPool.h>
#include <Poco/Redis/Array.h>
#include <Poco/Redis/Client.h>
#include <Poco/Redis/Command.h>
#include <Poco/Redis/PoolableConnectionFactory.h>

namespace palm {
class Redis {
 public:
  Redis(const boost::property_tree::ptree& config);
  std::shared_ptr<
      Poco::ObjectPool<Poco::Redis::Client, Poco::Redis::Client::Ptr>>
  open() const;

 private:
  std::string host;
  uint16_t port;
  std::string prefix;
  size_t pool_size;
};

class Cache {
 public:
  Cache(Poco::Redis::PooledConnection& db, const std::string& prefix)
      : db(db), prefix(prefix) {}
  void set(const std::string& key, const std::string& value,
           const std::chrono::seconds& ttl = std::chrono::days(1));
  Poco::Nullable<std::string> get(const std::string& key);
  std::string status();

 private:
  inline std::string key(const std::string& s) const {
    return this->prefix + "://" + s;
  }
  Poco::Redis::PooledConnection db;
  std::string prefix;
};

}  // namespace palm
