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

namespace tulip {
namespace portal {
std::shared_ptr<palm::portal::v1::Session> session(const httplib::Request& req);
std::shared_ptr<palm::portal::v1::Session> session(
    const boost::beast::http::request<boost::beast::http::string_body>& req);

std::shared_ptr<palm::portal::v1::Page> page(const httplib::Request& req);
struct Context {
  std::shared_ptr<palm::redis::Client> cache;
  std::shared_ptr<soci::connection_pool> db;
  std::shared_ptr<palm::rabbitmq::Client> queue;
  std::shared_ptr<palm::opensearch::Config> search;
  std::shared_ptr<inja::Environment> env;
  std::shared_ptr<::grpc::Channel> daisy;
};

template <typename H, typename Q, typename R>
inline boost::beast::http::message_generator json(
    const H& hnd, Context& ctx,
    const boost::beast::http::request<boost::beast::http::string_body>& req) {
  std::shared_ptr<palm::portal::v1::Session> ss = session(req);
  Q body;
  {
    const auto status =
        google::protobuf::util::JsonStringToMessage(req.body(), &body);
    if (!status.ok()) {
      return palm::http::bad_request(req, status.error_message());
    }
  }
  std::shared_ptr<R> res = hnd.execute(ctx, ss, body);
  if (res == nullptr) {
    return palm::http::internal_server_error(req);
  }
  return palm::http::json(req, *res);
}

template <typename H, typename Q, typename R>
inline boost::beast::http::message_generator html(
    const H& hnd, Context& ctx,
    const boost::beast::http::request<boost::beast::http::string_body>& req) {
  std::shared_ptr<palm::portal::v1::Session> ss = session(req);
  Q body;
  {
    const auto status =
        google::protobuf::util::JsonStringToMessage(req.body(), &body);
    if (!status.ok()) {
      return palm::http::bad_request(req, status.error_message());
    }
  }
  std::pair<std::string, std::shared_ptr<R>> res = hnd.execute(ctx, ss, body);
  if (res.second == nullptr) {
    return palm::http::internal_server_error(req);
  }
  return palm::http::html(req, res.first, *res.second);
}

}  // namespace portal

}  // namespace tulip
