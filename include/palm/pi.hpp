#pragma once

#include "palm/crypto.hpp"
#include "palm/orm.hpp"
#include "palm/tty.hpp"
#include "palm/utils.hpp"
#include "pi.grpc.pb.h"

namespace palm {
namespace pi {

class TtyService final : public palm::pi::v1::Tty::Service {
 public:
  TtyService(std::shared_ptr<Jwt> jwt, std::shared_ptr<palm::SerialPort> port)
      : jwt(jwt), port(port) {}
  ~TtyService() {}
  grpc::Status Write(grpc::ServerContext* context,
                     const palm::pi::v1::TtyRequest* request,
                     google::protobuf::Empty* response) override;

 private:
  std::shared_ptr<Jwt> jwt;
  std::shared_ptr<palm::SerialPort> port;
};
}  // namespace pi
}  // namespace palm
