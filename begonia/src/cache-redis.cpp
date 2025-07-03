#include "palm/cache.hpp"

#include <spdlog/spdlog.h>

std::shared_ptr<sw::redis::Redis> palm::redis::Node::open() const {
  spdlog::debug("open redis tcp://{}:{}/{}", this->_host, this->_port,
                this->_db);

  sw::redis::ConnectionOptions connection_options;
  connection_options.host = this->_host;
  connection_options.port = this->_port;
  if (this->_password) {
    connection_options.password = this->_password.value();
  }
  connection_options.db = this->_db;
  //   connection_options.socket_timeout = std::chrono::milliseconds(200);

  sw::redis::ConnectionPoolOptions pool_options;
  pool_options.size = this->_pool_size;
  //   pool_options.wait_timeout = std::chrono::milliseconds(100);
  //   pool_options.connection_lifetime = std::chrono::minutes(10);

  std::shared_ptr<sw::redis::Redis> it =
      std::make_shared<sw::redis::Redis>(connection_options, pool_options);
  {
    const auto v = it->ping();
    spdlog::debug("PING: {}", v);
  }
  return it;
}
