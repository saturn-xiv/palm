#pragma once

#include "monitoring.grpc.pb.h"
#include "palm/jwt.hpp"
#include "palm/rpc.hpp"
#include "palm/session.hpp"

namespace phlox {
void mount(httplib::Server& server, std::shared_ptr<palm::Jwt> jwt,
           std::shared_ptr<grpc::Channel> channel);

}  // namespace palm
