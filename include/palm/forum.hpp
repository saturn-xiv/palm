#pragma once

#include "forum.grpc.pb.h"
#include "palm/nut.hpp"

namespace palm {
namespace forum {

void mount(httplib::Server& srv);

}
}  // namespace palm
