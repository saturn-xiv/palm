#include "palm/orm.hpp"

#include <format>

std::shared_ptr<soci::session> palm::Sqlite3::open() const {
  std::shared_ptr<soci::session> db = std::make_shared<soci::session>(
      soci::sqlite3, std::format("db={} timeout={} shared_cache=true",
                                 this->_file, this->_timeout));
  db->set_logger(new palm::SociLogger());
  return db;
}
