#include "palm/network.hpp"

#include <ifaddrs.h>
#include <linux/if.h>
#include <netdb.h>
#include <netinet/ether.h>
#include <sys/ioctl.h>
#include <unistd.h>
#include <cstring>

std::vector<std::string> palm::network::interfaces() {
  struct ifaddrs *devices;
  getifaddrs(&devices);

  std::vector<std::string> items;

  for (struct ifaddrs *it = devices; it != nullptr; it = it->ifa_next) {
    if (it->ifa_addr && it->ifa_addr->sa_family == AF_PACKET) {
      std::string name = it->ifa_name;
      if (name == "lo") {
        continue;
      }
      items.push_back(name);
    }
  }

  freeifaddrs(devices);
  return items;
}

std::string palm::network::mac(const std::string &name) {
  int fd = socket(PF_INET, SOCK_DGRAM, IPPROTO_IP);

  struct ifreq ifr {};
  strcpy(ifr.ifr_name, name.c_str());
  ioctl(fd, SIOCGIFHWADDR, &ifr);
  close(fd);

  char mac[18];
  strcpy(mac, ether_ntoa((ether_addr *)ifr.ifr_hwaddr.sa_data));

  return mac;
}
