#pragma once

#include <algorithm>
#include <cstdint>
#include <sstream>
#include <string>
#include <vector>
#include <optional>

namespace palm {
namespace network {
class Ipv4 {
 public:
  Ipv4(const std::string& address, const std::string& netmask);

  void cidr(uint8_t v);
  void netmask(const std::string& v);
  void address(const std::string& v);
  // https://docs.netgate.com/pfsense/en/latest/network/cidr.html
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

struct Host {
  std::string mac;
  std::string ip;
  std::optional<std::string> vendor;
};
std::vector<Host> scan(const std::vector<std::string>& networks);
std::vector<std::string> interfaces();
std::string mac(const std::string& device);
bool is_wired_ethernet(const std::string& device);
}  // namespace network
}  // namespace palm
