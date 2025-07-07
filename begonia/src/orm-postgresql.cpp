#include "palm/orm.hpp"

#include <sstream>

std::string palm::PostgreSql::uri() const {
  std::stringstream ss;
  ss << "host=" << this->_host << " port=" << this->_port
     << " user=" << this->_user;
  if (this->_password) {
    ss << " password=" << this->_password.value();
  }
  ss << " dbname=" << this->_db_name << " sslmode=disable";
  return ss.str();
}

std::shared_ptr<soci::connection_pool> palm::PostgreSql::open(
    size_t pool_size) const {
  const auto url = this->uri();
  std::shared_ptr<soci::connection_pool> pool =
      std::make_shared<soci::connection_pool>(pool_size);
  for (size_t i = 0; i != pool_size; ++i) {
    soci::session& it = pool->at(i);
    it.open(soci::postgresql, url);
  }
  return pool;
}
