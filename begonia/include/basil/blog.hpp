#pragma once

#include "basil/wisteria.hpp"
#include "blog.grpc.pb.h"

namespace basil {
namespace blog {
namespace services {
class PageServiceImpl final : public basil::blog::v1::Page::Service {};
class PostServiceImpl final : public basil::blog::v1::Post::Service {};
}  // namespace services
}  // namespace blog
}  // namespace basil
