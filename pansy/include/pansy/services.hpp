#pragma once

#include "cups.grpc.pb.h"

#include <mutex>
#include <string>
#include <vector>

namespace pansy {
namespace services {
class CupsServiceImpl final : public palm::cups::v1::Cups::Service {
 public:
  CupsServiceImpl();

  grpc::Status Index(grpc::ServerContext* context,
                     const google::protobuf::Empty* request,
                     palm::cups::v1::CupsIndexResponse* reply) override;
  grpc::Status Print(grpc::ServerContext* context,
                     const palm::cups::v1::CupsPrintRequest* request,
                     google::protobuf::Empty* reply) override;

 private:
  std::mutex _mutex;
};
}  // namespace services
}  // namespace pansy
