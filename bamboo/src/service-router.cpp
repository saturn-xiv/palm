#include "bamboo/models.hpp"
#include "bamboo/network.hpp"
#include "bamboo/services.hpp"
#include "palm/network.hpp"
#include "palm/utils.hpp"

grpc::Status bamboo::services::RouterServiceImpl::SetEthernet(
    grpc::ServerContext* context,
    const palm::router::v1::RouterIndexEthernetResponse_Item* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  spdlog::info("save {}", request->device());
  {
    soci::transaction tr(*this->_db);
    const auto key = bamboo::network::key_of_interface(request->device());
    bamboo::dao::set(*this->_db, key, *request);
    tr.commit();
  }
  return grpc::Status::OK;
}
grpc::Status bamboo::services::RouterServiceImpl::IndexEthernet(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    palm::router::v1::RouterIndexEthernetResponse* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  const auto interfaces = palm::network::interfaces();
  for (const auto& device : interfaces) {
    const auto key = bamboo::network::key_of_interface(device);
    const auto mac = palm::network::mac(device);
    spdlog::debug("found network interface {}/{}", device, mac);
    auto it = reply->add_items();

    if (!bamboo::dao::get(*this->_db, key, it)) {
      it->set_device(device);
      it->set_mac(mac);
    }
  }
  return grpc::Status::OK;
}
grpc::Status bamboo::services::RouterServiceImpl::Reboot(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  palm::reboot();
  return grpc::Status::OK;
}
grpc::Status bamboo::services::RouterServiceImpl::Apply(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    google::protobuf::Empty* reply) {
  // TODO
  return grpc::Status::OK;
}
grpc::Status bamboo::services::RouterServiceImpl::FactoryReset(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    google::protobuf::Empty* reply) {
  // TODO
  return grpc::Status::OK;
}
