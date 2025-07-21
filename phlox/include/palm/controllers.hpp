#pragma once

#include "monitoring.grpc.pb.h"
#include "palm/jwt.hpp"
#include "palm/rpc.hpp"
#include "palm/session.hpp"

namespace palm {
void mount(httplib::Server& server, std::shared_ptr<palm::Jwt> jwt,
           std::shared_ptr<grpc::Channel> channel);
struct CurrentUser {
  inline static const std::string ISSUER = "phlox";
  inline static const std::string WEB_AUDIENCE = "web";
};
}  // namespace palm
