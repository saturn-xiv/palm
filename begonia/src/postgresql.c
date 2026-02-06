#include "third.h"

#include <stdlib.h>
#include <string.h>

int postgresql_timestamp(const char* timestamp, struct tm* time,
                         int64_t* microseconds) {
  memset(time, 0, sizeof(struct tm));
  if (strptime(timestamp, "%Y-%m-%d %H:%M:%S", time) == NULL) {
    return EXIT_FAILURE;
  }
  char* dot = strchr(timestamp, '.');
  if (dot != NULL) {
    if (sscanf(dot + 1, "%6d", microseconds) != 1) {
      return EXIT_FAILURE;
    }
  } else {
    *microseconds = 0;
  }
  return EXIT_SUCCESS;
}
