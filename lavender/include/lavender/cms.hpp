#pragma once

#include "cms.grpc.pb.h"
#include "lavender/portal.hpp"

namespace lavender {
namespace cms {
namespace services {
class PageServiceImpl final : public palm::cms::v1::Page::Service {};
}  // namespace services
}  // namespace cms
}  // namespace palm
