#pragma once

#include "basil/wisteria.hpp"
#include "cms.grpc.pb.h"

namespace basil {
namespace cms {
namespace services {
class PageServiceImpl final : public basil::cms::v1::Page::Service {};
}  // namespace services
}  // namespace cms
}  // namespace basil
