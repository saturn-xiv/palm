#include <palm/cache.hpp>

std::shared_ptr<sw::redis::RedisCluster> palm::Redis::open() const {
  sw::redis::ConnectionOptions options;
  options.host = this->_host;
  options.port = this->_port;

  sw::redis::ConnectionPoolOptions pool;
  pool.size = this->_pool_size;

  return std::make_shared<sw::redis::RedisCluster>(options, pool);
}
