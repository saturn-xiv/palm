#include "bamboo/network.hpp"
#include "palm/crypto.hpp"
#include "palm/theme.hpp"
#include "palm/utils.hpp"

#include <pugixml.hpp>

// https://www.linode.com/docs/guides/linux-router-and-ip-forwarding/
// https://www.digitalocean.com/community/tutorials/iptables-essentials-common-firewall-rules-and-commands
// sudo pacman -S dnsmasq man-pages net-tools iproute2 dnsutils inetutils
// sudo apt install iptables-persistent netfilter-persistent

static void setup_systemd_networkd(const palm::router::v1::Network& network,
                                   std::ostream& out) {
  spdlog::debug("render systemd-networkd");
  palm::render(R"TEMPLATE(
echo 'setup systemd networkd'
if [ -d /etc/systemd/network ]
then 
    rm -rv /etc/systemd/network
fi
mkdir -pv /etc/systemd/network

{% for dev, net in items -%}
echo "setup network for {{ dev }}"
cat >/etc/systemd/network/20-{{ dev }}.network <<EOF
[Match]
Name={{ dev }}

[Network]
{% if existsIn(net, "wan") -%}
Address={{ net.wan.address }}/{{ net.wan.cidr }}
Gateway={{ net.wan.gateway }}
{% for it in net.wan.dns -%}
DNS={{ it }}
{% endfor -%}
{% else if existsIn(net, "lan") -%}
Address={{ net.lan.address }}/{{ net.lan.cidr }}
{% endif -%}
EOF
{% endfor -%}

systemctl restart systemd-networkd
)TEMPLATE",
               network, out);
}
static void setup_dnsmasq(const palm::router::v1::Network& network,
                          std::ostream& out) {
  spdlog::debug("render dnsmasq");
  palm::render(R"TEMPLATE(
echo "setup dnsmasq"
systemctl stop dnsmasq
systemctl disable dnsmasq
rm -v /usr/lib/systemd/system/dnsmasq-*.service

{% for dev, net in items -%}
{% if existsIn(net, "lan") and existsIn(net.lan, "dhcp") -%}
echo "setup dnsmasq for {{ dev }}"
cat >/usr/lib/systemd/system/dnsmasq-{{ dev }}.service <<EOF
[Unit]
Description=dnsmasq - A lightweight DHCP and caching DNS server({{ dev }})
Documentation=man:dnsmasq(8)
After=network.target
Before=network-online.target nss-lookup.target
Wants=nss-lookup.target

[Service]
Type=simple
ExecStartPre=/usr/bin/dnsmasq --test -C /etc/dnsmasq-{{ dev }}.conf
ExecStart=/usr/bin/dnsmasq -k --user=dnsmasq -C /etc/dnsmasq-{{ dev }}.conf -x /tmp/dnsmasq.{{ dev }}.pid
Restart=on-failure
PrivateDevices=true
ProtectSystem=full

[Install]
WantedBy=multi-user.target
EOF

cat >/etc/dnsmasq-{{ dev }}.conf <<EOF
interface={{ dev }}
except-interface=lo
bind-interfaces
listen-address={{ net.lan.address }}

no-resolv
{% for it in net.lan.dhcp.dns -%}
server={{ it }}
{% endfor -%}

clear-on-reload

dhcp-range={{ net.lan.dhcp.begin }},{{ net.lan.dhcp.end }},1w
dhcp-option=option:dns-server,{{ net.lan.address }}
dhcp-option=option:router,{{ net.lan.address }}
dhcp-authoritative

{% for ip, it in net.lan.dhcp.reservedHosts -%}
dhcp-host={{ lower(it.mac) }},{{ it.name }},{{ ip }}
{% endfor -%}
EOF

systemctl daemon-reload
systemctl enable dnsmasq-{{ dev }}.service
systemctl restart dnsmasq-{{ dev }}.service
{% endif -%}
{% endfor -%}
)TEMPLATE",
               network, out);
}

static const std::string KERNEL_ENABLE_FORWARD = R"SHELL(
echo 'enable ipv4 forward'
cat >/etc/sysctl.d/100-router.conf <<EOF
net.ipv4.ip_forward = 1
net.ipv6.conf.all.forwarding = 1
EOF
systemctl restart systemd-sysctl
)SHELL";

static const std::string IPTABLES_CLEAR = R"SHELL(
echo 'clear firewall rules'
iptables -F
iptables -X
iptables -t nat -F
iptables -t nat -X
iptables -t mangle -F
iptables -t mangle -X
)SHELL";

static const std::string IPTABLES_SAVE = R"SHELL(
if [ -f /etc/iptables/iptables.rules ]
then
  mv /etc/iptables/iptables.rules /etc/iptables/$(date +"%Y%m%d%H%M%S").rules
fi
iptables-save -f /etc/iptables/iptables.rules
# iptables-restore /etc/iptables/iptables.rules
systemctl enable iptables
)SHELL";

static const std::string ECHO_DONE = "echo 'done.'";

static void set_firewall_public(const palm::router::v1::Network& network,
                                std::ostream& out) {
  spdlog::debug("render public firewall rules");
  palm::render(R"TEMPLATE(
echo 'setup iptables'
iptables -P INPUT ACCEPT
iptables -P OUTPUT ACCEPT
iptables -P FORWARD ACCEPT

{% for dev, net in items -%}
{% if existsIn(net, "lan") -%}
iptables -t nat -A POSTROUTING -s {{ net.lan.network }} -i {{ dev }} -j MASQUERADE
{% endif -%}
{% endfor -%}
)TEMPLATE",
               network, out);
}

static void setup_firewall(const palm::router::v1::Network& network,
                           std::ostream& out) {
  spdlog::debug("render firewall rules");
  palm::render(R"TEMPLATE(

echo 'setup iptables'
iptables -P INPUT DROP
iptables -P OUTPUT ACCEPT
iptables -P FORWARD DROP

# Allowing Loopback Connections
iptables -A INPUT -i lo -j ACCEPT

# Allowing All Incoming SSH
iptables -A INPUT -p tcp --dport 22 -m conntrack --ctstate NEW,ESTABLISHED -j ACCEPT

# Allow wlan ping
{% for dev, net in items -%}
{% if existsIn(net, "wan") -%}
iptables -A INPUT -i {{ dev }} -p icmp --icmp-type 8 -j ACCEPT
{% endif -%}
{% endfor -%}

# Allowing Internal Connections
{% for dev, net in items -%}
{% if existsIn(net, "lan") -%}
iptables -A INPUT -i {{ dev }} -j ACCEPT
{% endif -%}
{% endfor -%}

# Allowing Established and Related Incoming Connections
iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT

# Dropping Invalid Packets
iptables -A INPUT -m conntrack --ctstate INVALID -j DROP

# INPUT rules
{% for dev, net in items -%}
{% if existsIn(net, "wan") -%}
{% for it in net.wan.firewall.input -%}
iptables -A INPUT -i {{ dev }} -p {{ lower(it.protocol) }} --dport {{ it.port }} -m conntrack --ctstate NEW,ESTABLISHED -j ACCEPT
{% endfor -%}
{% endif -%}
{% endfor -%}

# WAN NAT rules
{% for dev, net in items -%}
{% if existsIn(net, "wan") -%}
{% for it in net.wan.firewall.nat -%}
iptables -t nat -A PREROUTING -i {{ dev }} -p {{ lower(it.protocol) }} --dport {{ it.port }} -j DNAT --to-destination {{ it.destination.ip }}:{{ it.destination.port }}
iptables -t nat -A POSTROUTING -p {{ lower(it.protocol) }} -d {{ it.destination.ip }} --dport {{ it.destination.port }} -j ACCEPT
{% endfor -%}
{% endif -%}
{% endfor -%}

# LAN NAT rules
{% for dev, net in items -%}
{% if existsIn(net, "lan") -%}
iptables -t nat -A POSTROUTING -s {{ net.lan.network }} -i {{ dev }} -j MASQUERADE
{% endif -%}
{% endfor -%}
)TEMPLATE",
               network, out);
}

void bamboo::network::apply(const palm::router::v1::Network& it, bool run) {
  const auto cur = palm::timestamp();
  {
    std::string clear = std::format("{}-clear.sh", cur);
    {
      spdlog::info("generate file {}", clear);
      std::ofstream out(clear);
      out << palm::bash::HEADER << palm::bash::REQUIRE_ROOT;
      setup_systemd_networkd(it, out);
      setup_dnsmasq(it, out);
      out << KERNEL_ENABLE_FORWARD << IPTABLES_CLEAR;
      set_firewall_public(it, out);
      out << IPTABLES_SAVE << std::endl
          << ECHO_DONE << std::endl
          << palm::bash::FOOTER;
      out.close();
    }
  }
  std::string apply = std::format("{}-apply.sh", cur);
  {
    spdlog::info("generate file {}", apply);
    std::ofstream out(apply);
    out << palm::bash::HEADER << palm::bash::REQUIRE_ROOT;
    setup_systemd_networkd(it, out);
    setup_dnsmasq(it, out);
    out << KERNEL_ENABLE_FORWARD << IPTABLES_CLEAR;
    setup_firewall(it, out);
    out << IPTABLES_SAVE << std::endl
        << ECHO_DONE << std::endl
        << palm::bash::FOOTER;
    out.close();
  }
  if (run) {
    spdlog::info("run script {}", apply);
    const auto& [status, out, err] = palm::shell("/bin/bash", {apply});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  spdlog::info("done.");
}

// https://docs.netgate.com/pfsense/en/latest/network/cidr.html
std::optional<uint8_t> bamboo::network::netmask_to_cidr(const std::string& s) {
  if (s == "255.255.255.255") {
    return 32;
  }
  if (s == "255.255.255.0") {
    return 24;
  }
  if (s == "255.255.254.0") {
    return 23;
  }
  return std::nullopt;
}

static void load_nmap_host(const pugi::xml_node& node,
                           bamboo::network::Host& host) {
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

std::vector<bamboo::network::Host> bamboo::network::scan(
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

  std::vector<bamboo::network::Host> items;
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
    bamboo::network::Host it;
    load_nmap_host(node.node(), it);
    if (!it.mac.empty() && !it.ip.empty()) {
      items.push_back(it);
    }
  }
  //   nmaprun/host/address addr addrtype
  return items;
}
