#pragma once

#include "blog.grpc.pb.h"
#include "lavender/portal.hpp"

namespace lavender {
namespace blog {
namespace services {
class PageServiceImpl final : public palm::blog::v1::Page::Service {};
class PostServiceImpl final : public palm::blog::v1::Post::Service {};
}  // namespace services
}  // namespace blog
}  // namespace lavender
