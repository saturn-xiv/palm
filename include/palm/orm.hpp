#pragma once

#include <string>

#include <iostream>
#include <vector>

#include <Poco/Data/Session.h>

namespace palm {
namespace orm {
struct Migration {
  int64_t version;
  std::string name;
  std::string up;
  std::string down;
};
}  // namespace orm
}  // namespace palm
