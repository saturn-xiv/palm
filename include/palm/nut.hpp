#pragma once

#include "auth.grpc.pb.h"
#include "nut.grpc.pb.h"
#include "palm/orm.hpp"
namespace palm {
namespace nut {
class SiteService final : public palm::nut::v1::Site::Service {
 public:
  grpc::Status About(grpc::ServerContext* context,
                     const google::protobuf::Empty* request,
                     palm::nut::v1::AboutResponse* reply) override;
  grpc::Status Install(grpc::ServerContext* context,
                       const palm::auth::v1::SignUpRequest* request,
                       google::protobuf::Empty* reply) override;
};
}  // namespace nut
}  // namespace palm
