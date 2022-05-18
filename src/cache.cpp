
#include "palm/cache.hpp"

#include <boost/log/trivial.hpp>

#include <Poco/Redis/Array.h>
#include <Poco/Redis/Command.h>

palm::Redis::Redis(const boost::property_tree::ptree& config) {
  this->host = config.get("redis.host", "127.0.0.1");
  this->port = config.get("redis.port", 6379);
  this->prefix = config.get<std::string>("redis.namespace");
  this->pool_size = config.get("redis.pool-size", 32);
}

std::shared_ptr<Poco::ObjectPool<Poco::Redis::Client, Poco::Redis::Client::Ptr>>
palm::Redis::open() const {
  Poco::Net::SocketAddress addr(this->host, this->port);
  Poco::PoolableObjectFactory<Poco::Redis::Client, Poco::Redis::Client::Ptr>
      factory(addr);
  std::shared_ptr<
      Poco::ObjectPool<Poco::Redis::Client, Poco::Redis::Client::Ptr>>
      pool = std::make_shared<
          Poco::ObjectPool<Poco::Redis::Client, Poco::Redis::Client::Ptr>>(
          factory, 2, this->pool_size);
  return pool;
}

void palm::Cache::set(const std::string& key, const std::string& value,
                      const std::chrono::seconds& ttl) {
  Poco::Redis::Array cmd;
  const auto k = this->key(key);
  const auto t = std::to_string(ttl.count());
  cmd << "SET" << k << value << "EX" << t;
  ((Poco::Redis::Client::Ptr)this->db)->execute<std::string>(cmd);
}

Poco::Nullable<std::string> palm::Cache::get(const std::string& key) {
  Poco::Redis::Array cmd;
  const auto k = this->key(key);
  cmd << "GET" << k;
  Poco::Redis::BulkString val =
      ((Poco::Redis::Client::Ptr)db)->execute<Poco::Redis::BulkString>(cmd);
  return val;
}

std::string palm::Cache::status() {
  Poco::Redis::Array cmd;
  cmd << "INFO";
  Poco::Redis::BulkString val =
      ((Poco::Redis::Client::Ptr)db)->execute<Poco::Redis::BulkString>(cmd);
  return val.value();
}

void palm::Cache::clear() {
  std::vector<std::string> items;
  {
    Poco::Redis::Array cmd;
    {
      const auto k = this->key("*");
      cmd << "KEYS" << k;
    }
    Poco::Redis::Array val =
        ((Poco::Redis::Client::Ptr)this->db)->execute<Poco::Redis::Array>(cmd);
    if (!val.isNull()) {
      for (auto i = 0; i < val.size(); i++) {
        const Poco::Redis::BulkString it = val.get<Poco::Redis::BulkString>(i);
        if (!it.isNull()) {
          const std::string k = it.value();
          items.push_back(k);
        }
      }
    }
  }
  Poco::Redis::Array cmd;
  cmd << "DEL" << items;
  ((Poco::Redis::Client::Ptr)this->db)->execute<Poco::Redis::Array>(cmd);
  // cmd << "FLUSHDB";
  // ((Poco::Redis::Client::Ptr)this->db)->execute<std::string>(cmd);
}

std::map<std::string, int64_t> palm::Cache::keys() {
  std::map<std::string, int64_t> items;
  Poco::Redis::Array cmd;
  {
    const auto k = this->key("*");
    cmd << "KEYS" << k;
  }
  Poco::Redis::Array val =
      ((Poco::Redis::Client::Ptr)this->db)->execute<Poco::Redis::Array>(cmd);
  if (!val.isNull()) {
    for (auto i = 0; i < val.size(); i++) {
      const Poco::Redis::BulkString it = val.get<Poco::Redis::BulkString>(i);
      if (!it.isNull()) {
        const std::string k = it.value();
        Poco::Redis::Array c;
        c << "TTL" << k;
        int64_t v = ((Poco::Redis::Client::Ptr)this->db)->execute<int64_t>(c);
        items[k] = v;
      }
    }
  }
  return items;
}
