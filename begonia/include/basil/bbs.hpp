#pragma once

#include "basil/wisteria.hpp"
#include "bbs.grpc.pb.h"

namespace basil {
namespace bbs {
namespace services {
class ForumServiceImpl final : public basil::bbs::v1::Forum::Service {};
class TopicServiceImpl final : public basil::bbs::v1::Topic::Service {};
class PostServiceImpl final : public basil::bbs::v1::Post::Service {};
}  // namespace services
}  // namespace bbs
}  // namespace basil
