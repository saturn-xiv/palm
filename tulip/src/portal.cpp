#include "tulip/portal.hpp"

#define PALM_DEFAULT_PAGE_SIZE 60
#define PALM_DEFAULT_PAGE_INDEX 1

std::shared_ptr<palm::portal::v1::Session> tulip::portal::session(
    const httplib::Request& req) {
  auto it = std::make_shared<palm::portal::v1::Session>();
  // TODO
  it->set_client_ip("n/a");
  return it;
}

std::shared_ptr<palm::portal::v1::Page> tulip::portal::page(
    const httplib::Request& req) {
  const std::string index = req.get_param_value("index");
  const std::string size = req.get_param_value("size");
  auto it = std::make_shared<palm::portal::v1::Page>();

  if (!index.empty()) {
    std::istringstream iss(index);
    int64_t v = 0;
    iss >> v;
    it->set_index(v);
  }
  if (!size.empty()) {
    std::istringstream iss(size);
    int64_t v = 0;
    iss >> v;
    it->set_size(v);
  }
  return it;
}
