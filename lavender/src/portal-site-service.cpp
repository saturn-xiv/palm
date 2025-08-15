#include "lavender/portal.hpp"

grpc::Status lavender::portal::services::SiteService::Timezones(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::portal::v1::SiteTimezonesResponse* reply) {
  for (auto& it : std::chrono::get_tzdb().zones) {
    reply->add_items(it.name());
  }
  return grpc::Status::OK;
}
grpc::Status lavender::portal::services::SiteService::Languages(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::portal::v1::SiteLanguagesResponse* reply) {
  // TODO
  return grpc::Status::OK;
}
grpc::Status lavender::portal::services::SiteService::Currencies(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::portal::v1::SiteCurrenciesResponse* reply) {
  // TODO
  return grpc::Status::OK;
}
