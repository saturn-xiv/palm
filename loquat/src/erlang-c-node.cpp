#include <loquat/env.hpp>
#include <loquat/erlang.hpp>
#include <loquat/version.hpp>

loquat::erlang::CNode::CNode(const std::string& nodename,
                             const std::string& cookie, uint16_t port) {
  spdlog::debug("start a erlang c-node({}, {}) and listening on :{}", nodename,
                cookie, port);
  // TODO
}

loquat::erlang::CNode::~CNode() {
  // TODO
}

void loquat::erlang::CNode::run() const {
  // TODO
}
