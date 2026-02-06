#ifndef PALM_HYACINTH_THIRD_H
#define PALM_HYACINTH_THIRD_H

#include <hiredis/hiredis.h>
#include <log4c.h>
#include <sodium.h>
#include <postgresql/libpq-fe.h>

void debug(const log4c_category_t* a_category, const char* a_format, ...);
void info(const log4c_category_t* a_category, const char* a_format, ...);
void warn(const log4c_category_t* a_category, const char* a_format, ...);
void error_(const log4c_category_t* a_category, const char* a_format, ...);

int match(const char* string, const char* pattern);

redisContext* redis_open(const char* host, uint16_t port);
int redis_ping(redisContext* client);
int redis_set(redisContext* client, const char* key, const uint8_t* value,
              size_t value_len, size_t ttl);
int redis_get(redisContext* client, const char* key, uint8_t* value,
              size_t buffer_len);

#endif
