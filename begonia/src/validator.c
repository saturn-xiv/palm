#include "third.h"

#include <regex.h>
#include <stdlib.h>

int match(const char* original, const char* pattern) {
  log4c_category_t* logger = log4c_category_get("regex");
  regex_t regex;
  if (regcomp(&regex, pattern, REG_EXTENDED)) {
    error_(logger, "couldn't compile %s\n", pattern);
    return EXIT_FAILURE;
  }

  int status = regexec(&regex, original, 0, NULL, 0);
  if (!status) {
    return EXIT_SUCCESS;
  }
  if (status != REG_NOMATCH) {
    char buf[255];
    regerror(status, &regex, buf, sizeof(buf));
    error_(logger, "regex match failed: %s\n", buf);
  }

  regfree(&regex);
  return EXIT_FAILURE;
}
