
#include "palm/cache.hpp"

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
