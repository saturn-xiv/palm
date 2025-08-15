#include "palm/theme.hpp"
#include "palm/version.hpp"
#include "phlox/services.hpp"

grpc::Status phlox::monitoring::services::SiteServiceImpl::Layout(
    grpc::ServerContext* context,
    const palm::monitoring::v1::SiteLayoutRequest* request,
    palm::monitoring::v1::SiteLayoutResponse* reply) {
  const std::string en_us = "en-US";
  phlox::CurrentUser current_user(context);
  {
    auto name = current_user.name(this->_jwt);
    if (!name) {
      const grpc::Status it(grpc::StatusCode::UNAUTHENTICATED, "invalid token");
      return it;
    }
    reply->mutable_user()->set_name(name.value());
  }
  reply->set_locale(en_us);
  reply->add_available_languages(en_us);
  reply->set_build_time(palm::BUILD_TIME);
  reply->set_git_version(palm::GIT_VERSION);
  {
    const auto now = google::protobuf::util::TimeUtil::GetCurrentTime();
    reply->mutable_created_at()->CopyFrom(now);
  }
  return grpc::Status::OK;
}
