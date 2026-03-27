#include "palm/version.hpp"
#include "tulip/portal.hpp"

grpc::Status tulip::portal::rpc::service::Site::Heartbeat(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::portal::v1::SiteHeartbeatResponse* reply) {
  // TODO test db/cache/s3/queue/daisy
  {
    auto now = google::protobuf::util::TimeUtil::GetCurrentTime();
    auto it = new google::protobuf::Timestamp();
    it->set_seconds(now.seconds());
    it->set_nanos(now.nanos());
    reply->set_allocated_created_at(it);
  }
  {
    const std::string it =
        std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME);
    reply->set_version(it);
  }
  return grpc::Status::OK;
}
