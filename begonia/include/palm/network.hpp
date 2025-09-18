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
  Ipv4(const std::string& address, const std::string& netmask);

  void cidr(uint8_t v);
  void netmask(const std::string& v);
  void address(const std::string& v);

  uint8_t cidr() const;
  inline std::string netmask() const { return this->_netmask; }
  inline std::string address() const { return this->_address; }
  std::string network() const;
  std::string broadcast() const;
  std::string default_gateway() const;
  std::pair<std::string, std::string> addresses() const;

 private:
  std::string _address;
  std::string _netmask;
  std::string _router;
};
}  // namespace network
}  // namespace palm
