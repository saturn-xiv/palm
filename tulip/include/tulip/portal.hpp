#pragma once

#include "email.grpc.pb.h"
#include "palm/cache.hpp"
#include "palm/http.hpp"
#include "palm/orm.hpp"
#include "palm/queue.hpp"
#include "palm/rpc.hpp"
#include "palm/search.hpp"
#include "palm/snowflake.hpp"
#include "palm/utils.hpp"
#include "portal.grpc.pb.h"
#include "rbac.grpc.pb.h"
#include "s3.grpc.pb.h"
#include "sms.grpc.pb.h"
#include "tex.grpc.pb.h"
#include "wechat-pay.grpc.pb.h"

#include <google/protobuf/util/json_util.h>
#include <google/protobuf/util/time_util.h>
#include <httplib.h>

namespace tulip {
namespace portal {
struct Context {
  std::shared_ptr<palm::redis::Client> cache;
  std::shared_ptr<soci::connection_pool> db;
  std::shared_ptr<palm::rabbitmq::Client> queue;
  std::shared_ptr<::grpc::Channel> daisy;
};
}  // namespace portal

}  // namespace tulip
