#pragma once

#include "bbs.grpc.pb.h"
#include "palm/wisteria.hpp"

namespace palm {
namespace bbs {
namespace services {
class ForumServiceImpl final : public palm::bbs::v1::Forum::Service {};
class TopicServiceImpl final : public palm::bbs::v1::Topic::Service {};
class PostServiceImpl final : public palm::bbs::v1::Post::Service {};
}  // namespace services
}  // namespace bbs
}  // namespace palm
