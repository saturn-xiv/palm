#include "palm/portal.hpp"

grpc::Status palm::portal::services::SiteService::Timezones(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::portal::v1::SiteTimezonesResponse* reply)  {
  // TODO
  return grpc::Status::OK;
}
grpc::Status palm::portal::services::SiteService::Languages(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::portal::v1::SiteLanguagesResponse* reply)  {
  // TODO
  return grpc::Status::OK;
}
grpc::Status palm::portal::services::SiteService::Currencies(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::portal::v1::SiteCurrenciesResponse* reply)  {
  // TODO
  return grpc::Status::OK;
}
