#include "palm/cache.hpp"

#include <boost/log/trivial.hpp>

std::shared_ptr<sw::redis::Redis> palm::redis::Node::open() const {
  BOOST_LOG_TRIVIAL(debug) << "open redis tcp://" << this->_host << ":"
                           << this->_port << "/" << +(this->_db);

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
    BOOST_LOG_TRIVIAL(debug) << v;
  }
  return it;
}
