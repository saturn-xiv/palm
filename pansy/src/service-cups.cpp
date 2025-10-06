#include "pansy/services.hpp"

#include <cups/cups.h>
#include <spdlog/spdlog.h>

static std::vector<std::tuple<std::string, std::optional<std::string>>>
    gl_all_available_destinations;

static int find_all_available_destinations(void* user_data, unsigned flags,
                                           cups_dest_t* dest) {
  std::tuple<std::string, std::optional<std::string>> it =
      std::make_tuple(dest->name, std::nullopt);
  if (dest->instance) {
    std::get<1>(it) = dest->instance;
  }
  gl_all_available_destinations.push_back(it);
  return 1;
}

grpc::Status pansy::services::CupsServiceImpl::Index(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::cups::v1::CupsIndexResponse* reply) {
  std::lock_guard<std::mutex> guard(this->_mutex);
  gl_all_available_destinations.clear();
  cupsEnumDests(CUPS_DEST_FLAGS_NONE, 1000, NULL, 0, 0,
                find_all_available_destinations, NULL);
  return grpc::Status::OK;
}
grpc::Status pansy::services::CupsServiceImpl::Print(
    grpc::ServerContext* context,
    const palm::cups::v1::CupsPrintRequest* request,
    google::protobuf::Empty* reply) {
  std::lock_guard<std::mutex> guard(this->_mutex);
  // TODO
  return grpc::Status::OK;
}
pansy::services::CupsServiceImpl::CupsServiceImpl() {
  spdlog::debug("cups version v{}", CUPS_VERSION);
}
