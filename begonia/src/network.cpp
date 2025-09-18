#include "palm/network.hpp"

#include <exception>

#define PALM_MIN_CIDR 1
#define PALM_MAX_CIDR 32

static inline int count_set_bits(unsigned int n) {
  int count = 0;
  while (n > 0) {
    n &= (n - 1);  // Brian Kernighan's algorithm
    count++;
  }
  return count;
}

palm::network::Ipv4::Ipv4(const std::string& mask, const std::string& router) {
  // TODO
}

void palm::network::Ipv4::address(const std::string& v) {}

uint8_t palm::network::Ipv4::cidr() const {
  std::istringstream ss(this->_netmask);
  std::string str;
  unsigned int rst = 0;
  for (int i = 0; i < 4; ++i) {
    std::getline(ss, str, '.');
    rst = (rst << 8) | std::stoul(str);
  }
  const auto v = count_set_bits(rst);
  if (v < PALM_MIN_CIDR || v > PALM_MAX_CIDR) {
    throw std::invalid_argument("invalid netmask");
  }
  return static_cast<uint8_t>(v);
}

void palm::network::Ipv4::cidr(uint8_t v) {
  if (v < PALM_MIN_CIDR || v > PALM_MAX_CIDR) {
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
