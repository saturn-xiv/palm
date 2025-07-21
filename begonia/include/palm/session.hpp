#pragma once

#include "palm/jwt.hpp"

#include <grpc/grpc.h>
#include <httplib.h>

namespace palm {
class Session {
 public:
  Session(grpc::ServerContext* context);
  Session(const httplib::Request& request);

  static void init(const httplib::Request& request,
                   grpc::ClientContext* context);

 protected:
  std::optional<std::string> _token;
  std::string _locale;
  std::optional<std::string> _client_ip;
};
}  // namespace palm
