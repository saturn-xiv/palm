#include "palm/cache.hpp"

palm::redis::Connection::~Connection() {
  if (this->context != nullptr) {
    BOOST_LOG_TRIVIAL(debug) << "release redis context";
    redisFree(this->context);
    this->context = nullptr;
  }
}

std::vector<std::pair<std::string, int>> palm::redis::Connection::keys() {
  std::vector<std::pair<std::string, int>> items;

  const auto keys = this->key("*");
  auto reply =
      (redisReply*)redisCommand(this->context, "KEYS %s", keys.c_str());

  for (auto i = 0; i < reply->elements; i++) {
    std::string key = reply->element[i]->str;
    auto re = (redisReply*)redisCommand(this->context, "TTL %s", key.c_str());
    int ttl = re->integer;
    freeReplyObject(re);
    items.push_back(std::make_pair(key, ttl));
  }
  freeReplyObject(reply);
  return items;
}

void palm::redis::Connection::set(
    const std::string& key_, const std::string& val,
    const std::optional<std::chrono::seconds> ttl) {
  const auto key = this->key(key_);
  redisReply* reply;
  if (ttl && ttl > std::chrono::steady_clock::duration::zero()) {
    reply =
        (redisReply*)redisCommand(this->context, "SET %s %s EX %d", key.c_str(),
                                  val.c_str(), ttl.value().count());
  } else {
    reply = (redisReply*)redisCommand(this->context, "SET %s %s", key.c_str(),
                                      val.c_str());
  }
  BOOST_LOG_TRIVIAL(debug) << reply->str;
  freeReplyObject(reply);
}

std::string palm::redis::Connection::get(const std::string& key_) {
  const auto key = this->key(key_);
  auto reply = (redisReply*)redisCommand(this->context, "GET %s", key.c_str());
  const std::string val = reply->str;
  BOOST_LOG_TRIVIAL(debug) << val;
  freeReplyObject(reply);
  return val;
}

void palm::redis::Connection::heatbeat() {
  auto reply = (redisReply*)redisCommand(this->context, "PING");
  BOOST_LOG_TRIVIAL(debug) << reply->str;
  freeReplyObject(reply);
}

void palm::redis::Connection::clear() {
  auto reply = (redisReply*)redisCommand(this->context, "FLUSHDB");
  BOOST_LOG_TRIVIAL(debug) << reply->str;
  freeReplyObject(reply);
}

std::shared_ptr<palm::redis::Connection> palm::redis::Config::build() {
  auto ctx = redisConnect(this->host.c_str(), this->port);
  if (ctx == nullptr) {
    throw std::invalid_argument("can't open redis");
  }
  if (ctx->err) {
    throw std::invalid_argument(ctx->errstr);
  }
  auto it = std::make_shared<palm::redis::Connection>(ctx, this->zone);
  return it;
}
