#include "palm/orm.hpp"

std::shared_ptr<soci::connection_pool> palm::PostgreSql::open() const {
  spdlog::debug("open postgresql://{}@{}:{}/{} with {} connections",
                this->_user, this->_host, this->_port, this->_db_name,
                this->_pool_size);
  const auto url = this->uri();
  std::shared_ptr<soci::connection_pool> pool =
      std::make_shared<soci::connection_pool>(this->_pool_size);
  for (size_t i = 0; i < this->_pool_size; ++i) {
    soci::session& it = pool->at(i);
    it.open(soci::postgresql, url);
    it.set_logger(new palm::SociLogger());
  }
  return pool;
}

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
