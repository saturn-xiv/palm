#include "bamboo/application.hpp"
#include "bamboo/network.hpp"
#include "bamboo/services.hpp"
#include "palm/network.hpp"
#include "palm/utils.hpp"
#include "palm/version.hpp"

#include <grpcpp/health_check_service_interface.h>
#include <grpcpp/security/server_credentials.h>
#include <sodium.h>
#include <argparse/argparse.hpp>

void bamboo::Application::launch(int argc, char* argv[]) {
  bool debug;

  std::string rpc_config_file;
  std::string rpc_listen_host;
  int rpc_listen_port;

  std::string apply_input_file;
  bool apply_run;

  std::string sample_output_file;

  std::string scan_config_file;

  argparse::ArgumentParser program(
      "bamboo", std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME));
  program.add_description("A smart router inspired by OpenWrt.");
  program.add_epilog(palm::PROJECT_HOME);
  program.add_argument("-d", "--debug")
      .flag()
      .store_into(debug)
      .help("run on debug mode");

  argparse::ArgumentParser rpc_command("rpc");
  rpc_command.add_description("start a gRPC server");
  rpc_command.add_argument("-H", "--host")
      .default_value("127.0.0.1")
      .store_into(rpc_listen_host);
  rpc_command.add_argument("-p", "--port")
      .default_value(8080)
      .store_into(rpc_listen_port);
  rpc_command.add_argument("-c", "--config")
      .default_value("config.toml")
      .store_into(rpc_config_file)
      .help("configuration file");

  argparse::ArgumentParser reboot_command("reboot");
  reboot_command.add_description("reboot the system");

  argparse::ArgumentParser scan_command("scan");
  scan_command.add_description("scan the internal hosts");
  scan_command.add_argument("-c", "--config")
      .default_value("config.toml")
      .store_into(scan_config_file)
      .help("configuration file");

  argparse::ArgumentParser apply_command("apply");
  apply_command.add_description("apply from configuration");
  apply_command.add_argument("-i", "--input")
      .default_value("config.json")
      .store_into(apply_input_file)
      .help("configuration file");
  apply_command.add_argument("-r", "--run")
      .flag()
      .store_into(apply_run)
      .help("run it after generate the script file");

  argparse::ArgumentParser sample_command("sample");
  sample_command.add_description("generate a sample resource file");
  sample_command.add_argument("-o", "--output")
      .default_value("config.json")
      .store_into(sample_output_file)
      .help("configuration file");

  program.add_subparser(rpc_command);
  program.add_subparser(scan_command);
  program.add_subparser(reboot_command);
  program.add_subparser(sample_command);
  program.add_subparser(apply_command);
  program.parse_args(argc, argv);

  if (program.is_subcommand_used(reboot_command)) {
    palm::init(debug);
    this->reboot();
    return;
  }

  if (program.is_subcommand_used(apply_command)) {
    palm::init(debug);
    this->apply(apply_input_file, apply_run);
    return;
  }
  if (program.is_subcommand_used(sample_command)) {
    palm::init(debug);
    this->sample(sample_output_file);
    return;
  }

  if (program.is_subcommand_used(rpc_command)) {
    palm::init(debug);
    spdlog::info("load configuration from {}", rpc_config_file);
    toml::table config = toml::parse_file(rpc_config_file);
    if (program.is_subcommand_used(rpc_command)) {
      this->rpc_server(config, rpc_listen_host, rpc_listen_port);
      return;
    }
  }
  if (program.is_subcommand_used(scan_command)) {
    palm::init(debug);
    spdlog::info("load configuration from {}", scan_config_file);
    toml::table config = toml::parse_file(scan_config_file);
    this->scan(config);
    return;
  }
  std::cout << program << std::endl;
}

void bamboo::Application::rpc_server(const toml::table& config,
                                     const std::string& host, uint16_t port) {
  if (palm::is_stopped()) {
    return;
  }

  const std::string address = std::format("{}:{}", host, port);

  auto db = this->db(config);
  auto jwt = this->jwt(config);
  auto aes = this->aes(config);

  {
    soci::transaction tr(*db);
    const std::string installed_at = "site.installed-at";
    const auto it = bamboo::dao::get(*db, installed_at);
    if (!it) {
      spdlog::warn("empty database, will be setup it at first");
      bamboo::dao::administrator::save(*db, "admin", "123456");
      {
        const auto now = google::protobuf::util::TimeUtil::GetCurrentTime();
        bamboo::dao::set(*db, installed_at, now);
      }
    }
    tr.commit();
  }

  bamboo::services::AdministratorServiceImpl administrator_service(db, jwt);
  bamboo::services::RouterServiceImpl router_service(db, jwt);
  bamboo::services::UserServiceImpl user_service(db, aes, jwt);

  grpc::EnableDefaultHealthCheckService(true);

  grpc::ServerBuilder builder;
  builder.AddListeningPort(address, grpc::InsecureServerCredentials());

  {
    builder.RegisterService(&administrator_service);
    builder.RegisterService(&router_service);
    builder.RegisterService(&user_service);
  }

  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
  spdlog::info("listen a gRPC server on tcp://{}:{}", host, port);

  server->Wait();
}
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
    it->set_address("x.x.x.x");
    it->set_netmask("255.255.255.0");
    it->set_cidr(24);
    it->set_gateway("x.x.x.1");
    it->add_dns("8.8.8.8");
    it->add_dns("8.8.4.4");
    it->set_weight(6);
    it->set_description("Wan");
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
  deleted_at TIMESTAMP,
  version INTEGER NOT NULL DEFAULT 0, 
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_hosts_mac ON idx_hosts(mac);
CREATE INDEX IF NOT EXISTS idx_hosts_ip ON idx_hosts(ip);
CREATE INDEX IF NOT EXISTS idx_hosts_name ON idx_hosts(name);
CREATE INDEX IF NOT EXISTS idx_hosts_vendor ON idx_hosts(vendor);
)SQL";

    *db << R"SQL(
CREATE TABLE IF NOT EXISTS settings (
  id INTEGER PRIMARY KEY,
  "key" VARCHAR(255) NOT NULL,
  value BLOB NOT NULL,  
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
