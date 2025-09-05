#pragma once

#include "palm/theme.hpp"
#include "router.grpc.pb.h"

namespace bamboo {
namespace network {
void apply(const palm::router::v1::Network& it, bool run = false);
std::optional<uint8_t> netmask_to_cidr(const std::string& s);
struct Host {
  std::string mac;
  std::string ip;
  std::optional<std::string> vendor;
};
std::vector<Host> scan(const std::vector<std::string>& networks);
}  // namespace network
}  // namespace bamboo
