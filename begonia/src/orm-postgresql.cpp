#include "palm/orm.hpp"

#include <sstream>

std::string palm::PostgreSql::uri() {
  std::stringstream ss;
  ss << "host=" << this->_host << " port=" << this->_port
     << " user=" << this->_user;
  if (this->_password) {
    ss << " password=" << this->_password.value();
  }
  ss << " dbname=" << this->_db_name << " sslmode=disable";
  return ss.str();
}
