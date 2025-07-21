#include "palm/services.hpp"
#include "palm/session.hpp"
#include "palm/version.hpp"

grpc::Status palm::monitoring::services::SiteServiceImpl ::Layout(
    grpc::ServerContext* context,
    const palm::monitoring::v1::SiteLayoutRequest* request,
    palm::monitoring::v1::SiteLayoutResponse* reply) {
  const std::string en_us = "en-US";
  //   TODO set current-user
  reply->set_locale(en_us);
  reply->add_available_languages(en_us);
  reply->set_build_time(palm::BUILD_TIME);
  reply->set_git_version(palm::GIT_VERSION);
  return grpc::Status::OK;
}
