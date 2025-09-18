#include "palm/network.hpp"

#include <exception>

palm::network::Ipv4::Ipv4(const std::string& mask, const std::string& router) {}

void palm::network::Ipv4::address(const std::string& v) {}

uint16_t palm::network::Ipv4::cidr() const {}

void palm::network::Ipv4::cidr(uint8_t v) {
  if (v < 0 || v > 32) {
    throw std::invalid_argument("invalid CIDR prefix");
  }

  const unsigned int mask = (0xFFFFFFFF << (32 - v));
  std::stringstream ss;
  for (int i = 3; i >= 0; --i) {
    ss << std::to_string((mask >> (i * 8)) & 0xFF);
    if (i > 0) {
      ss << ".";
    }
  }
  this->_netmask = ss.str();
}
void palm::network::Ipv4::netmask(const std::string& v) {}
