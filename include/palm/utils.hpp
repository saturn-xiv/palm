#pragma once

#include "palm/env.hpp"

namespace palm {

template <typename T>
std::string to_string(T t) {
  std::stringstream ss;
  ss << t;
  return ss.str();
}

inline bool is_stopped() { return std::filesystem::exists(".stop"); }

void reboot();

class WatchDog {
 public:
  WatchDog();
  ~WatchDog();
  void feed();
  int get_timeout();
  void set_timeout(const int seconds = 5);

 private:
  int fd;
};

}  // namespace palm
