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
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  const auto interfaces = palm::network::interfaces();

  google::protobuf::Arena arena;
  auto network =
      google::protobuf::Arena::Create<palm::router::v1::Network>(&arena);

  {
    auto items = network->mutable_items();
    for (const auto& device : interfaces) {
      const auto key = bamboo::network::key_of_interface(device);
      const auto mac = palm::network::mac(device);
      spdlog::debug("found network interface {}/{}", device, mac);
      palm::router::v1::RouterIndexEthernetResponse_Item it;

      if (!bamboo::dao::get(*this->_db, key, &it)) {
        continue;
      }
      if (!it.enable()) {
        spdlog::debug("network interface {} is disables", device);
        continue;
      }
      if (it.has_wan()) {
        auto net =
            google::protobuf::Arena::Create<palm::router::v1::Network_Item>(
                &arena);
        auto wan = net->mutable_wan();
        wan->set_name(it.name());
        wan->set_description(it.description());
        // wan->set_cidr(static_cast<uint32_t>(ip.cidr()));
        // wan->set_address(it.wan())
        (*items)[device] = *net;
        if (it.wan().has_dhcp()) {
          // auto dhcp =
        } else if (it.wan().has_static_()) {
        }
        // palm::network::Ipv4 ip(it.wan().address(), it.wan().netmask());

        continue;
      }
      if (it.has_lan()) {
        palm::network::Ipv4 ip(it.lan().address(), it.lan().netmask());
        auto net =
            google::protobuf::Arena::Create<palm::router::v1::Network_Item>(
                &arena);
        auto lan = net->mutable_lan();
        lan->set_address(ip.address());
        lan->set_netmask(ip.netmask());
        lan->set_cidr(static_cast<uint32_t>(ip.cidr()));
        lan->set_network(ip.network());
        lan->set_blacklist_mode(true);
        lan->set_name(it.name());
        lan->set_description(it.description());
        {
          auto dhcp = lan->mutable_dhcp();
          switch (it.lan().region()) {
            case palm::router::v1::Region::China:
              dhcp->add_dns("223.5.5.5");
              dhcp->add_dns("223.6.6.6");
              break;
            default:
              dhcp->add_dns("8.8.8.8");
              dhcp->add_dns("8.8.4.4");
              break;
          }
          {
            auto addresses = ip.addresses();
            dhcp->set_begin(addresses.front());
            dhcp->set_end(addresses.back());
          }
          {
            auto hosts = dhcp->mutable_reserved_hosts();
            const auto items = bamboo::dao::host::all(*this->_db);
            boost::fusion::for_each(items, [hosts, &arena](auto const& host) {
              if (!host.fixed) {
                return;
              }
              if (host.deleted_at) {
                return;
              }
              auto it = google::protobuf::Arena::Create<
                  palm::router::v1::Lan_Dhcp_Host>(&arena);
              it->set_mac(host.mac);
              it->set_name(host.name);
              (*hosts)[host.ip] = *it;
            });
          }
        }
        (*items)[device] = *net;
        continue;
      }
    }
  }
  return grpc::Status::OK;
}
grpc::Status bamboo::services::RouterServiceImpl::FactoryReset(
    grpc::ServerContext* context, const google::protobuf::Empty* request,
    google::protobuf::Empty* reply) {
  if (!current_administrator(context, this->_jwt)) {
    return MUST_SIGNED_IN;
  }
  bamboo::router::factory_reset(true);
  return grpc::Status::OK;
}
