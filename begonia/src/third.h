#ifndef PALM_HYACINTH_THIRD_H
#define PALM_HYACINTH_THIRD_H

#include <log4c.h>

void log(const log4c_category_t* a_category, int a_priority,
         const char* a_format, ...) {
  va_list args;
  va_start(args, a_format);
  log4c_category_vlog(a_category, a_priority, a_format, args);
  va_end(args);
}

#endif
