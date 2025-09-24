#include "palm/network.hpp"
#include "palm/validator.hpp"

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

static inline unsigned int ip_to_int(const std::string& ip_str) {
  std::istringstream iss(ip_str);
  std::string octet;
  unsigned int ip_int = 0;
  for (int i = 0; i < 4; ++i) {
    std::getline(iss, octet, '.');
    ip_int = (ip_int << 8) | std::stoul(octet);
  }
  return ip_int;
}

static inline std::string int_to_ip(unsigned int ip_int) {
  std::string ip_str;
  for (int i = 0; i < 4; ++i) {
    ip_str = std::to_string(ip_int & 255) + (i == 0 ? "" : ".") + ip_str;
    ip_int >>= 8;
  }
  return ip_str;
}

static std::pair<unsigned int, unsigned int> network_and_broadcast(
    const std::string& _address, const std::string& _netmask) {
  unsigned int address = ip_to_int(_address);
  unsigned int netmask = ip_to_int(_netmask);

  unsigned int network = address & netmask;
  unsigned int wildcard_mask = ~netmask;
  unsigned int broadcast = network | wildcard_mask;
  if (broadcast - network < 2) {
    throw std::invalid_argument("invalid address and netmask");
  }
  return {network, broadcast};
}

palm::network::Ipv4::Ipv4(const std::string& address,
                          const std::string& netmask) {
  this->address(address);
  this->netmask(netmask);
}

std::string palm::network::Ipv4::network() const {
  const auto it = network_and_broadcast(this->_address, this->_netmask);
  return int_to_ip(it.first);
}
std::string palm::network::Ipv4::broadcast() const {
  const auto it = network_and_broadcast(this->_address, this->_netmask);
  return int_to_ip(it.second);
}
std::string palm::network::Ipv4::default_gateway() const {
  const auto it = network_and_broadcast(this->_address, this->_netmask);
  return int_to_ip(it.first + 1);
}
std::vector<std::string> palm::network::Ipv4::addresses() const {
  const auto it = network_and_broadcast(this->_address, this->_netmask);
  std::vector<std::string> items;
  for (int i = it.first + 2; i < it.second; i++) {
    const auto ip = int_to_ip(i);
    items.push_back(ip);
  }
  return items;
  // return {int_to_ip(it.first + 2), int_to_ip(it.second - 1)};
}
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
void palm::network::Ipv4::netmask(const std::string& v) {
  const auto it = palm::validator::ipv4(v);
  this->_netmask = it.value();
}
void palm::network::Ipv4::address(const std::string& v) {
  const auto it = palm::validator::ipv4(v);
  this->_address = it.value();
}

/*
en: Indicates a wired Ethernet interface.
p: Denotes a PCI Express device.
0s3: These numbers refer to the device's location:
  The first number (e.g., 0) is the bus number.
  The letter s stands for a hotplug slot.
  The last number (e.g., 3) is the slot number.
*/
bool palm::network::is_wired_ethernet(const std::string& device) {
  return device.starts_with("en");
}
