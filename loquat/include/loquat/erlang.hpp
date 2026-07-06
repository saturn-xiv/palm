#pragma once

#include <cstdint>
#include <string>

#include <ei.h>

namespace loquat {
namespace erlang {
class CNode {
 public:
  CNode(const std::string& nodename, const std::string& cookie, uint16_t port);
  ~CNode();

  void run() const;
  std::vector<std::string> global_names() const;

 private:
  void receive() const;

  ei_cnode _node;
  int _sock_fd;
  int _pub;

  inline static const std::string SERVICE_NAME = "loquat";
};
}  // namespace erlang
}  // namespace loquat
