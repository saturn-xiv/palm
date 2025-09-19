#pragma once

#include "palm/theme.hpp"
#include "router.grpc.pb.h"

namespace bamboo {
namespace network {
// FIXME remove
void apply(const palm::router::v1::Network& it, bool run = false);
// FIXME remove
std::optional<uint8_t> netmask_to_cidr(const std::string& s);

}  // namespace network
}  // namespace bamboo
