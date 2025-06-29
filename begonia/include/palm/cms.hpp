#pragma once

#include "palm/wisteria.hpp"
#include "cms.grpc.pb.h"

namespace palm {
namespace cms {
namespace services {
class PageServiceImpl final : public palm::cms::v1::Page::Service {};
}  // namespace services
}  // namespace cms
}  // namespace palm
