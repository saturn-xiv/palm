#ifndef PALM_HYACINTH_THIRD_H
#define PALM_HYACINTH_THIRD_H

#include <log4c.h>

void debug(const log4c_category_t* a_category, const char* a_format, ...) {
  va_list args;
  va_start(args, a_format);
  log4c_category_vlog(a_category, LOG4C_PRIORITY_DEBUG, a_format, args);
  va_end(args);
}
void info(const log4c_category_t* a_category, const char* a_format, ...) {
  va_list args;
  va_start(args, a_format);
  log4c_category_vlog(a_category, LOG4C_PRIORITY_INFO, a_format, args);
  va_end(args);
}
void warn(const log4c_category_t* a_category, const char* a_format, ...) {
  va_list args;
  va_start(args, a_format);
  log4c_category_vlog(a_category, LOG4C_PRIORITY_WARN, a_format, args);
  va_end(args);
}
void error_(const log4c_category_t* a_category, const char* a_format, ...) {
  va_list args;
  va_start(args, a_format);
  log4c_category_vlog(a_category, LOG4C_PRIORITY_ERROR, a_format, args);
  va_end(args);
}
#endif
