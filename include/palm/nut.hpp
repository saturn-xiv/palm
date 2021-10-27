#pragma once

#include "nut.grpc.pb.h"
#include "palm/orm.hpp"

namespace palm {
namespace nut {

void mount(httplib::Server& srv);

}  // namespace nut
}  // namespace palm
