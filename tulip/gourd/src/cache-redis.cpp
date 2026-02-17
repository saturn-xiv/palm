#include <palm/cache.hpp>

std::shared_ptr<palm::redis::Client> palm::redis::Config::open() const {
  spdlog::debug("open redis cluster tcp://{}:{}", this->_host, this->_port);
  sw::redis::ConnectionOptions options;
  options.host = this->_host;
  options.port = this->_port;

  sw::redis::ConnectionPoolOptions pool_options;
  pool_options.size = this->_pool_size;

  return std::make_shared<palm::redis::Client>(
      std::make_unique<sw::redis::RedisCluster>(options, pool_options),
      this->_namespace);
}
