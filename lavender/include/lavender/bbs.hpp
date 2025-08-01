#pragma once

#include "bbs.grpc.pb.h"
#include "lavender/portal.hpp"

namespace lavender {
namespace bbs {
namespace services {
class ForumServiceImpl final : public palm::bbs::v1::Forum::Service {};
class TopicServiceImpl final : public palm::bbs::v1::Topic::Service {};
class PostServiceImpl final : public palm::bbs::v1::Post::Service {};
}  // namespace services
}  // namespace bbs
}  // namespace lavender
