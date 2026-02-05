#include "third.h"

#include <stdlib.h>
#include <string.h>

redisContext* redis_open(const char* host, uint16_t port) {
  log4c_category_t* logger = log4c_category_get("redis");
  debug(logger, "open redis tcp://%s:%d\n", host, port);
  struct timeval timeout = {1, 500000};
  redisContext* client = redisConnectWithTimeout(host, port, timeout);
  if (client == NULL) {
    error_(logger, "can't allocate redis context\n");
    return NULL;
  }
  if (client->err) {
    error_(logger, "connect redis: %s\n", client->errstr);
    redisFree(client);
    return NULL;
  }
  return client;
}

static inline int _check_string_reply(const log4c_category_t* logger,
                                      redisContext* client, redisReply* reply) {
  if (reply == NULL) {
    error_(logger, "empty reply: %s\n", client->errstr);
    return EXIT_FAILURE;
  }
  //   if (reply->type == REDIS_REPLY_STRING) {
  //     return EXIT_FAILURE;
  //   }
  return EXIT_SUCCESS;
}

int redis_ping(redisContext* client) {
  log4c_category_t* logger = log4c_category_get("redis");
  debug(logger, "ping");
  redisReply* reply = redisCommand(client, "PING");
  int code = _check_string_reply(logger, client, reply);
  if (code == EXIT_SUCCESS) {
    debug(logger, "PING: %s\n", reply->str);
  }
  freeReplyObject(reply);
  return code;
}

int redis_set(redisContext* client, const char* key, const uint8_t* value,
              size_t value_len, size_t ttl) {
  log4c_category_t* logger = log4c_category_get("redis");
  debug(logger, "set %s %d", key, ttl);
  redisReply* reply = NULL;
  if (ttl == 0) {
    reply = redisCommand(client, "SET %s %b", key, value, value_len);
  } else {
    reply = redisCommand(client, "SETEX %s %d %b", key, ttl, value, value_len);
  }
  int code = _check_string_reply(logger, client, reply);
  if (code == EXIT_SUCCESS) {
    debug(logger, "SET: %s\n", reply->str);
  }
  freeReplyObject(reply);
  return code;
}

int redis_get(redisContext* client, const char* key, uint8_t* value,
              size_t buffer_len) {
  log4c_category_t* logger = log4c_category_get("redis");
  debug(logger, "get %s", key);
  redisReply* reply = redisCommand(client, "GET %s", key);
  int code = _check_string_reply(logger, client, reply);
  if (code != EXIT_SUCCESS) {
    freeReplyObject(reply);
    return -1;
  }
  debug(logger, "GET: %d bytes\n", reply->len);
  if (reply->len > buffer_len) {
    error_(logger, "not enough space %d\n", buffer_len);
    freeReplyObject(reply);
    return -1;
  }
  memcpy(value, reply->str, reply->len);
  freeReplyObject(reply);
  return reply->len;
}
