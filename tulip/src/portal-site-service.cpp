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

  reply->set_version(palm::GIT_VERSION);
  return grpc::Status::OK;
}
