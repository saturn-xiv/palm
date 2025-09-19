#include "palm/crypto.hpp"
#include "palm/network.hpp"
#include "palm/utils.hpp"

#include <pugixml.hpp>

static inline void load_nmap_host(const pugi::xml_node& node,
                                  palm::network::Host& host) {
  for (const pugi::xml_node& it : node.children("address")) {
    const std::string addr = it.attribute("addr").value();
    const std::string addr_type = it.attribute("addrtype").value();
    if (addr_type == "ipv4") {
      host.ip = addr;
      continue;
    }
    if (addr_type == "mac") {
      host.mac = addr;
      const auto vendor = it.attribute("vendor");
      if (vendor) {
        host.vendor = vendor.value();
      }

      continue;
    }
  }
}

std::vector<palm::network::Host> palm::network::scan(
    const std::vector<std::string>& networks) {
  const auto tmp = std::format("/tmp/{}.xml", palm::timestamp());

  {
    std::vector<std::string> args = {"-oX", tmp, "-sn"};
    args.insert(args.end(), networks.begin(), networks.end());
    const auto& [status, out, err] = palm::shell("/usr/bin/nmap", args);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return {};
    }
    spdlog::debug("{}", out);
  }

  std::vector<palm::network::Host> items;
  pugi::xml_document doc;

  {
    spdlog::debug("parse file {}", tmp);
    pugi::xml_parse_result rst = doc.load_file(tmp.c_str());
    if (!rst) {
      spdlog::error("failed to parse xml {}", rst.description());
      return {};
    }
  }

  pugi::xpath_node_set nodes = doc.select_nodes("/nmaprun/host");
  for (const pugi::xpath_node& node : nodes) {
    palm::network::Host it;
    load_nmap_host(node.node(), it);
    if (!it.mac.empty() && !it.ip.empty()) {
      items.push_back(it);
    }
  }
  //   nmaprun/host/address addr addrtype
  return items;
}
