#pragma once

#include <algorithm>
#include <cstdint>
#include <sstream>
#include <string>
#include <vector>

namespace palm {
namespace network {
class Ipv4 {
 public:
  Ipv4(const std::string& mask, const std::string& router);
  void address(const std::string& v);
  void cidr(uint8_t v);
  void netmask(const std::string& v);
  uint8_t cidr() const;
  inline std::string netmask() const { return this->_netmask; }

 private:
  std::string _address;
  std::string _netmask;
  std::string _router;
};
}  // namespace network
}  // namespace palm
