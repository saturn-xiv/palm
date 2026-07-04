#pragma once

#include <cstdint>
#include <string>

namespace loquat {
class Application {
 public:
  Application() {}

  void launch(int argc, char** argv) const;

 private:
  void start_erlang_c_node(const std::string& nodename,
                           const std::string& cookie, uint16_t port) const;
};
}  // namespace loquat
