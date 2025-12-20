#include "iris/utils.hpp"

#include <pwd.h>
#include <sys/types.h>
#include <unistd.h>

std::filesystem::path iris::home() {
  struct passwd *pw = getpwuid(getuid());
  const char *it = pw->pw_dir;
  return it;
}
