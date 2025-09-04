#include "bamboo/network.hpp"
#include "palm/crypto.hpp"
#include "palm/theme.hpp"
#include "palm/utils.hpp"

// https://www.linode.com/docs/guides/linux-router-and-ip-forwarding/
// sudo pacman -S dnsmasq man-pages net-tools iproute2 dnsutils inetutils

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
dhcp-host={{ it.mac }},{{ it.name }},{{ ip }}
{% endfor -%}
EOF

systemctl daemon-reload
systemctl enable dnsmasq-{{ dev }}.service
systemctl restart dnsmasq-{{ dev }}.service
{% endif %}
{% endfor %}
)TEMPLATE",
               network, out);
}
static void setup_firewall(const palm::router::v1::Network& network,
                           std::ostream& out) {
  spdlog::debug("render firewall");
  palm::render(R"TEMPLATE(

echo 'enable ipv4 forward'
cat >/etc/sysctl.d/100-router.conf <<EOF
net.ipv4.ip_forward = 1
EOF
systemctl restart systemd-sysctl

echo 'setup iptables'
iptables -F
iptables -X
iptables -t nat -F
iptables -t nat -X
iptables -t mangle -F
iptables -t mangle -X
iptables -P INPUT ACCEPT
iptables -P OUTPUT ACCEPT
iptables -P FORWARD ACCEPT
iptables -A FORWARD -j ACCEPT

iptables -t nat -s 192.168.11.0/24 -A POSTROUTING -j MASQUERADE
iptables -t nat -s 192.168.12.0/24 -A POSTROUTING -j MASQUERADE

iptables-save > /tmp/firewall-$(date +"%Y%m%d%H%M%S").sh
)TEMPLATE",
               network, out);
}

void bamboo::network::apply(const palm::router::v1::Network& it, bool run) {
  std::string tmp = std::format("{}.sh", palm::timestamp());
  {
    spdlog::info("generate file {}", tmp);
    std::ofstream out(tmp);
    out << palm::bash::HEADER << palm::bash::REQUIRE_ROOT;
    setup_systemd_networkd(it, out);
    setup_dnsmasq(it, out);
    setup_firewall(it, out);
    out << "echo 'done.'" << palm::bash::FOOTER;
    out.close();
  }
  if (run) {
    spdlog::info("run script {}", tmp);
    const auto& [status, out, err] = palm::shell("/bin/bash", {tmp});
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
