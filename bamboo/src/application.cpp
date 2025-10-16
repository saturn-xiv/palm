#include "bamboo/application.hpp"
#include "bamboo/network.hpp"
#include "bamboo/services.hpp"
#include "palm/network.hpp"
#include "palm/utils.hpp"

#include <sodium.h>

void bamboo::Application::reboot() {
  spdlog::warn("will reboot the system");
  palm::reboot();
}
void bamboo::Application::apply(const std::string& input, bool run) {
  spdlog::info("load network configuration from {}", input);
  palm::router::v1::Network it;
  {
    std::ifstream fs(input);
    std::string buf((std::istreambuf_iterator<char>(fs)),
                    std::istreambuf_iterator<char>());
    const auto st = palm::from_json(buf, &it);
    if (!st.ok()) {
      spdlog::error("failed to parse json {}", st.error_message());
      return;
    }
  }
  bamboo::network::apply(it, run);
}
void bamboo::Application::sample(const std::string& output) {
  if (std::filesystem::exists(output)) {
    spdlog::error("file {} already exists", output);
    return;
  }

  google::protobuf::Arena arena;
  auto network =
      google::protobuf::Arena::Create<palm::router::v1::Network>(&arena);
  auto items = network->mutable_items();
  {
    auto eth =
        google::protobuf::Arena::Create<palm::router::v1::Network_Item>(&arena);
    auto it = eth->mutable_wan();

    it->set_name("wan-0");
    {
      auto jt = it->mutable_static_();
      jt->set_address("x.x.x.x");
      jt->set_netmask("255.255.255.0");
      jt->set_cidr(24);
      jt->set_gateway("x.x.x.1");
      jt->add_dns("8.8.8.8");
      jt->add_dns("8.8.4.4");
    }
    it->set_weight(6);
    it->set_description("Wan");
    /*
    {
      auto fw = it->mutable_firewall();
      fw->set_ping(true);
      {
        auto in = fw->add_input();
        in->set_protocol(palm::router::v1::Firewall_Protocol_Tcp);
        in->set_port(22);
      }
      {
        auto in = fw->add_input();
        in->set_protocol(palm::router::v1::Firewall_Protocol_Tcp);
        in->set_port(80);
      }
      {
        auto nat = fw->add_nat();
        nat->set_protocol(palm::router::v1::Firewall_Protocol_Tcp);
        nat->set_port(10022);
        {
          auto dest = nat->mutable_destination();
          dest->set_ip("192.168.2.10");
          dest->set_port(22);
        }
      }
      {
        auto nat = fw->add_nat();
        nat->set_protocol(palm::router::v1::Firewall_Protocol_Tcp);
        nat->set_port(10080);
        {
          auto dest = nat->mutable_destination();
          dest->set_ip("192.168.2.10");
          dest->set_port(80);
        }
      }
    }
    */

    (*items)["eth0"] = *eth;
  }

  {
    auto eth =
        google::protobuf::Arena::Create<palm::router::v1::Network_Item>(&arena);
    auto it = eth->mutable_lan();

    it->set_name("lan");
    it->set_address("192.168.1.1");
    it->set_netmask("255.255.255.0");
    it->set_blacklist_mode(true);
    it->set_cidr(24);
    it->set_network("192.168.1.0/24");
    it->set_description("Lan");
    {
      auto dhcp = it->mutable_dhcp();
      dhcp->set_begin("192.168.1.2");
      dhcp->set_end("192.168.1.254");
      dhcp->add_dns("8.8.8.8");
      dhcp->add_dns("8.8.4.4");
    }

    (*items)["eth1"] = *eth;
  }
  {
    auto eth =
        google::protobuf::Arena::Create<palm::router::v1::Network_Item>(&arena);
    auto it = eth->mutable_lan();

    it->set_name("dmz");
    it->set_address("192.168.2.1");
    it->set_netmask("255.255.255.0");
    it->set_blacklist_mode(true);
    it->set_cidr(24);
    it->set_network("192.168.2.0/24");
    {
      auto dhcp = it->mutable_dhcp();
      dhcp->set_begin("192.168.2.2");
      dhcp->set_end("192.168.2.254");
      dhcp->add_dns("8.8.8.8");
      dhcp->add_dns("8.8.4.4");
      {
        auto hosts = dhcp->mutable_reserved_hosts();
        {
          auto ih =
              google::protobuf::Arena::Create<palm::router::v1::Lan_Dhcp_Host>(
                  &arena);
          ih->set_mac("xx:xx:xx:xx:xx:10");
          ih->set_name("host-10");
          (*hosts)["192.168.2.10"] = *ih;
        }
        {
          auto ih =
              google::protobuf::Arena::Create<palm::router::v1::Lan_Dhcp_Host>(
                  &arena);
          ih->set_mac("xx:xx:xx:xx:xx:11");
          ih->set_name("host-11");
          (*hosts)["192.168.2.11"] = *ih;
        }
        {
          auto ih =
              google::protobuf::Arena::Create<palm::router::v1::Lan_Dhcp_Host>(
                  &arena);
          ih->set_mac("xx:xx:xx:xx:xx:12");
          ih->set_name("host-12");
          (*hosts)["192.168.2.12"] = *ih;
        }
      }
    }

    it->set_description("Dmz");

    (*items)["eth2"] = *eth;
  }

  auto body = palm::to_json(*network, true);
  if (!body) {
    return;
  }
  {
    spdlog::info("generate file {}", output);
    std::ofstream out(output);
    out << body.value();
    out.close();
  }
  spdlog::info("done.");
}

std::optional<std::vector<uint8_t>> bamboo::Application::secrets(
    const toml::table& config) {
  const auto it = config["secrets"].value<std::string>();
  if (!it) {
    return std::nullopt;
  }
  const auto buf = palm::base64::from_string(it.value());
  // if (buf.size() != crypto_auth_KEYBYTES) {
  //   spdlog::error("invalid mac key length({},{})", buf.size(),
  //                 crypto_auth_KEYBYTES);
  //   return std::nullopt;
  // }
  if (buf.size() != crypto_secretbox_KEYBYTES) {
    spdlog::error("invalid encrypt key length({},{})", buf.size(),
                  crypto_secretbox_KEYBYTES);
    return std::nullopt;
  }
  return buf;
}

std::shared_ptr<soci::session> bamboo::Application::db(
    const toml::table& config) {
  const std::string db_file =
      config["db-file"].value_or<std::string>("db.sqlite3");
  palm::Sqlite3 cfg(db_file);
  auto db = cfg.open();
  {
    soci::transaction tr(*db);

    // COULDN'T put all sql together
    *db << R"SQL(
CREATE TABLE IF NOT EXISTS logs (
  id INTEGER PRIMARY KEY,
  message TEXT NOT NULL,  
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
)SQL";

    *db << R"SQL(
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY,
  real_name VARCHAR(31) NOT NULL,
  wifi_nickname VARCHAR(31) NOT NULL,
  wifi_password BLOB NOT NULL,
  wifi_begin_at TIMESTAMP NOT NULL,
  wifi_end_at TIMESTAMP NOT NULL,
  wechat VARCHAR(63),
  email VARCHAR(63),
  address VARCHAR(127),
  phone VARCHAR(15),
  version INTEGER NOT NULL DEFAULT 0, 
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_wiki_nickname ON users(wiki_nickname);
CREATE INDEX IF NOT EXISTS idx_users_realname ON users(wiki_realname);
)SQL";

    *db << R"SQL(
CREATE TABLE IF NOT EXISTS hosts (
  id INTEGER PRIMARY KEY,
  user_id INTEGER,
  mac CHAR(17) NOT NULL,
  ip VARCHAR(45) NOT NULL,
  name VARCHAR(63) NOT NULL,
  vendor VARCHAR(63) NOT NULL,
  fixed BOOLEAN NOT NULL DEFAULT FALSE,
  description VARCHAR(1023) NOT NULL DEFAULT '',
  deleted_at TIMESTAMP,
  version INTEGER NOT NULL DEFAULT 0, 
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_hosts_mac ON idx_hosts(mac);
CREATE INDEX IF NOT EXISTS idx_hosts_ip ON idx_hosts(ip);
CREATE INDEX IF NOT EXISTS idx_hosts_name ON idx_hosts(name);
CREATE INDEX IF NOT EXISTS idx_hosts_vendor ON idx_hosts(vendor);
CREATE INDEX IF NOT EXISTS idx_hosts_description ON idx_hosts(description);
)SQL";

    *db << R"SQL(
CREATE TABLE IF NOT EXISTS settings (
  id INTEGER PRIMARY KEY,
  "key" VARCHAR(255) NOT NULL,
  value TEXT NOT NULL,  
  version INTEGER NOT NULL DEFAULT 0, 
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_settings_key ON settings("key");
)SQL";

    tr.commit();
  }
  return db;
}
std::shared_ptr<palm::Jwt> bamboo::Application::jwt(const toml::table& config) {
  spdlog::debug("load jwt");
  auto cfg = config["jwt"].as_table();
  if (cfg == nullptr) {
    spdlog::error("missing jwt part");
    return nullptr;
  }
  std::shared_ptr<palm::Jwt> it =
      std::make_shared<palm::Jwt>((*cfg)["key"].value<std::string>().value());
  return it;
}
std::shared_ptr<palm::Aes> bamboo::Application::aes(const toml::table& config) {
  spdlog::debug("load aes");
  auto cfg = config["aes"].as_table();
  if (cfg == nullptr) {
    spdlog::error("missing aes part");
    return nullptr;
  }
  std::shared_ptr<palm::Aes> it =
      std::make_shared<palm::Aes>((*cfg)["key"].value<std::string>().value(),
                                  (*cfg)["iv"].value<std::string>().value());
  return it;
}
std::shared_ptr<palm::HMac> bamboo::Application::hmac(
    const toml::table& config) {
  spdlog::debug("load hmac");
  auto cfg = config["hmac"].as_table();
  if (cfg == nullptr) {
    spdlog::error("missing hmac part");
    return nullptr;
  }
  std::shared_ptr<palm::HMac> it =
      std::make_shared<palm::HMac>((*cfg)["key"].value<std::string>().value());
  return it;
}

void bamboo::Application::scan(const toml::table& config) {
  auto db = this->db(config);
  std::vector<std::string> networks;
  for (const auto& name : palm::network::interfaces()) {
    const auto mac = palm::network::mac(name);
    spdlog::debug("found network interface {} {}", name, mac);
    const std::string key = bamboo::network::key_of_interface(name);
    palm::router::v1::RouterIndexEthernetResponse_Item it;
    if (!bamboo::dao::get(*db, key, &it)) {
      continue;
    }
    if (!it.enable()) {
      spdlog::warn("{} isn't enabled", name);
      continue;
    }
    if (!it.has_lan()) {
      spdlog::debug("ignore {}", name);
      continue;
    }
    palm::network::Ipv4 ip(it.lan().address(), it.lan().netmask());
    const std::string net = std::format("{}/{}", ip.network(), ip.cidr());
    networks.push_back(net);
  }

  if (networks.empty()) {
    spdlog::warn("couldn't found available network interfaces");
    return;
  }
  auto items = palm::network::scan(networks);

  {
    soci::transaction tr(*db);
    for (const auto& it : items) {
      spdlog::debug("found host({}, {}, {})", it.mac, it.ip,
                    it.vendor.value_or(""));
      bamboo::dao::host::save(*db, it.mac, "", it.ip, it.vendor.value_or(""));
    }
    tr.commit();
  }
  spdlog::info("done.");
}
