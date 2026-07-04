#pragma once

#include <cstdint>
#include <string>

namespace loquat {
namespace erlang {
class CNode {
 public:
  CNode(const std::string& nodename, const std::string& cookie, uint16_t port);
  ~CNode();

  void run()const;
 private:
};
}  // namespace erlang
}  // namespace loquat
