#pragma once

#include "palm/wisteria.hpp"
#include "blog.grpc.pb.h"

namespace palm {
namespace blog {
namespace services {
class PageServiceImpl final : public palm::blog::v1::Page::Service {};
class PostServiceImpl final : public palm::blog::v1::Post::Service {};
}  // namespace services
}  // namespace blog
}  // namespace palm
