#include "bamboo/application.hpp"
#include "bamboo/services.hpp"
#include "palm/utils.hpp"
#include "palm/version.hpp"

#include <grpcpp/health_check_service_interface.h>
#include <grpcpp/security/server_credentials.h>
#include <argparse/argparse.hpp>

void bamboo::Application::launch(int argc, char* argv[]) {
  bool debug;

  std::string rpc_config_file;
  std::string rpc_listen_host;
  int rpc_listen_port;

  std::string apply_input_file;
  bool apply_run;

  std::string sample_output_file;

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

  argparse::ArgumentParser apply_command("apply");
  apply_command.add_description("apply");
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
      .store_into(apply_input_file)
      .help("configuration file");

  program.add_subparser(rpc_command);
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
  auto hmac = this->hmac(config);

  bamboo::services::AdministratorServiceImpl administrator_service(db, aes,
                                                                   hmac, jwt);
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
  // TODO
}
void bamboo::Application::sample(const std::string& output) {
  // TODO
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
  deleted_at TIMESTAMP,
  version INTEGER NOT NULL DEFAULT 0, 
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_hosts_mac ON idx_hosts(mac);
CREATE INDEX IF NOT EXISTS idx_hosts_ip ON idx_hosts(ip);
CREATE INDEX IF NOT EXISTS idx_hosts_name ON idx_hosts(name);
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
