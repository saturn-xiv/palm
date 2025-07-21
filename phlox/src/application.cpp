#include "palm/application.hpp"
#include "palm/controllers.hpp"
#include "palm/filesystem.hpp"
#include "palm/podman.hpp"
#include "palm/rpc.hpp"
#include "palm/services.hpp"
#include "palm/utils.hpp"
#include "palm/version.hpp"

#include <unistd.h>
#include <format>
#include <iostream>
#include <regex>
#include <stdexcept>

#include <boost/algorithm/string.hpp>
#include <boost/asio/ip/host_name.hpp>
#include <boost/optional/optional.hpp>
#include <boost/range/iterator_range.hpp>

#include <grpcpp/health_check_service_interface.h>
#include <argparse/argparse.hpp>

static std::shared_ptr<soci::session> open_db(const toml::table& config) {
  const std::string db_file =
      config["db-file"].value_or<std::string>("db.sqlite3");
  palm::Sqlite3 cfg(db_file);
  auto db = cfg.open();
  {
    soci::transaction tr(*db);
    *db << R"SQL(
CREATE TABLE IF NOT EXISTS containers (
  id INTEGER PRIMARY KEY, 
  host VARCHAR(127) NOT NULL,
  uid VARCHAR(127) NOT NULL,
  status VARCHAR(15) NOT NULL,
  last_fetch_logs_at TIMESTAMP,
  version INTEGER NOT NULL DEFAULT 0,
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_containers_uid ON containers(uid);
CREATE INDEX IF NOT EXISTS idx_containers_host ON containers(host);
CREATE INDEX IF NOT EXISTS idx_containers_status ON containers(status);
)SQL";
    tr.commit();
  }
  return db;
}

static std::shared_ptr<palm::Jwt> open_jwt(const toml::table& config) {
  auto cfg = config["jwt"].as_table();
  if (cfg == nullptr) {
    spdlog::error("missing jwt part");
    return nullptr;
  }
  std::shared_ptr<palm::Jwt> it =
      std::make_shared<palm::Jwt>((*cfg)["key"].value<std::string>().value());
  return it;
}
static std::shared_ptr<grpc::Channel> open_backend(const toml::table& config) {
  auto node = config["backend"].as_table();
  if (node == nullptr) {
    spdlog::error("missing backend part");
    return nullptr;
  }
  palm::GRpcClient cfg(*node);
  return cfg.open();
}
static std::shared_ptr<palm::opensearch::Client> open_opensearch(
    const toml::table& config) {
  std::shared_ptr<palm::opensearch::Client> it =
      std::make_shared<palm::opensearch::Client>(
          *(config["opensearch"].as_table()));
  {
    const auto res = it->cluster_health();
    spdlog::debug("cluster {} ({} nodes) {}", res->cluster_name,
                  res->number_of_nodes, res->status);
  }
  return it;
}

static void launch_podman_logs(const toml::table& config) {
  if (palm::is_stopped()) {
    return;
  }
  auto db = open_db(config);
  auto search = open_opensearch(config);
  // TODO
}
static void launch_podman_stats(const toml::table& config) {
  if (palm::is_stopped()) {
    return;
  }
  auto db = open_db(config);
  auto search = open_opensearch(config);
  const auto items = palm::podman::stats();
  for (const auto& it : items) {
    spdlog::debug("find container {}({})", it.name, it.id);
  }
  // TODO
}
static void launch_podman_ps(const toml::table& config) {
  if (palm::is_stopped()) {
    return;
  }
  auto db = open_db(config);
  auto search = open_opensearch(config);
  const auto items = palm::podman::ps();
  for (const auto& it : items) {
    spdlog::debug("find container {}({})", it.Id,
                  boost::algorithm::join(it.Names, ","));
  }
  // TODO
}

static void generate_etc(const std::string& domain) {
  const auto etc = std::filesystem::path("etc") / domain;
  if (std::filesystem::exists(etc)) {
    spdlog::warn("folder {} exists", etc.string());
    return;
  }

  spdlog::debug("generate folder {}", etc.string());
  std::filesystem::create_directories(etc);

  {
    auto file = etc / "nginx.conf";
    spdlog::info("generate file {}", file.string());
    nlohmann::json data = {
        {"domain", domain},
        {
            "api",
            {{"hosts", {"192.168.21", "192.168.22", "192.168.23"}},
             {"port", 8080}},
        },
    };

    spdlog::debug("args:\n{}", data.dump(4));
    const std::string tpl = R"NGINX(
# -----------------------------------------------------------------------------

# https://nginx.org/en/docs/http/ngx_http_upstream_module.html
upstream api_{{ domain }} {
## for host in api.hosts
  server {{ host }}:{{ api.port }};
## endfor
}

# https://pro.ant.design/docs/deploy/#use-nginx
server {
  listen 80;

  server_name {{ domain }};
  access_log /var/log/nginx/{{ domain }}.access.log;
  error_log  /var/log/nginx/{{ domain }}.error.log;

  gzip on;
  gzip_comp_level 9;
  gzip_min_length 1k;
  gzip_types text/plain text/css application/xml application/javascript;
  gzip_vary on;
  client_max_body_size 128M;

  location /my/ {
    alias /usr/share/palm/phlox/dashboard/;
    try_files $uri $uri/ /my/index.html;

    location ~* \.(css|js|png|jpg|jpeg|gif|gz|svg|mp4|ogg|ogv|webm|htc|xml|woff)$ {
      access_log off;
      expires max;
    }
  }
  
  location / {
    proxy_set_header X-Forwarded-Proto http;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header Host $http_host;
    proxy_redirect off;
    proxy_pass http://api_{{ domain }};
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
  }
}

# -----------------------------------------------------------------------------
)NGINX";
    std::ofstream out(file);
    inja::render_to(out, tpl, data);
  }

  {
    const auto file = etc / std::format("api.{}.conf", domain);
    spdlog::info("generate file {}", file.string());

    nlohmann::json data = {{"domain", domain}, {"port", 8080}};

    spdlog::debug("args:\n{}", data.dump(4));
    const std::string tpl = R"SYSTEMD(
[Unit]
Description=HTTP api service for {{ domain }}
After=rpc.{{ domain }}.service

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/var/lib/{{ domain }}
ExecStart=/usr/bin/phlox -c /etc/palm/{{ domain }}-http.toml -p {{ port }}
# or always, on-abort, on-failure, etc
Restart=always
RestartSec=10s

[Install]
WantedBy=multi-user.target
)SYSTEMD";
    std::ofstream out(file);
    inja::render_to(out, tpl, data);
  }

  {
    const auto file = etc / std::format("rpc.{}.conf", domain);
    spdlog::info("generate file {}", file.string());

    nlohmann::json data = {{"domain", domain}, {"port", 9090}};

    spdlog::debug("args:\n{}", data.dump(4));
    const std::string tpl = R"SYSTEMD(
[Unit]
Description=gRPC service for {{ domain }}
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/var/lib/{{ domain }}
ExecStart=/usr/bin/phlox -c /etc/{{ domain }}-rpc.toml rpc -p {{ port }}
# or always, on-abort, on-failure, etc
Restart=always
RestartSec=10s

[Install]
WantedBy=multi-user.target
)SYSTEMD";
    std::ofstream out(file);
    inja::render_to(out, tpl, data);
  }

  spdlog::info("done.");
}

static void start_log_watcher(const toml::table& config, bool stdin,
                              const std::vector<std::string>& original_files) {
  if (palm::is_stopped()) {
    return;
  }

  std::shared_ptr<palm::opensearch::Client> search =
      std::make_shared<palm::opensearch::Client>(config);
  {
    auto res = search->cluster_health();
    spdlog::debug("{} {}", res->cluster_name, res->status);
  }
  if (!search->index_exists<palm::monitoring::logging::Item>()) {
    const auto props = palm::monitoring::logging::Item::properties();
    search->create_index<palm::monitoring::logging::Item>(2, 1, props);
  }
  palm::monitoring::LoggingScratcher scratcher;

  if (stdin) {
    spdlog::info("listen from STDIN stream");
    std::shared_ptr<palm::monitoring::logging::Source> it =
        std::make_shared<palm::monitoring::logging::StdinSource>();
    scratcher.register_(it);
  }
  {
    std::shared_ptr<palm::monitoring::logging::FilesystemNotify> it =
        std::make_shared<palm::monitoring::logging::FilesystemNotify>();
    const std::set<std::string> items(original_files.begin(),
                                      original_files.end());
    for (const auto& file : items) {
      it->register_(file);
    }

    scratcher.register_(it);
  }
  scratcher.launch(search);
}

static void start_http_server(const std::string& host, uint16_t port,
                              const toml::table& config) {
  if (palm::is_stopped()) {
    return;
  }

  auto jwt = open_jwt(config);
  auto channel = open_backend(config);

  httplib::Server server;
  palm::set_logger(server);
  palm::mount(server, jwt, channel);

  spdlog::info("listen a HTTP server on tcp://{}:{}", host, port);
  server.listen(host, port);
}
static void start_rpc_server(const std::string& host, uint16_t port,
                             const toml::table& config) {
  if (palm::is_stopped()) {
    return;
  }

  const std::string address = std::format("{}:{}", host, port);

  auto search = open_opensearch(config);
  auto jwt = open_jwt(config);

  palm::monitoring::services::SiteServiceImpl site_service(jwt, search);
  palm::monitoring::services::PodmanServiceImpl podman_service(jwt, search);
  palm::monitoring::services::FileSystemServiceImpl file_system_service(jwt,
                                                                        search);

  grpc::EnableDefaultHealthCheckService(true);
  // TODO
  // grpc::reflection::InitProtoReflectionServerBuilderPlugin();

  grpc::ServerBuilder builder;
  builder.AddListeningPort(address, grpc::InsecureServerCredentials());

  {
    builder.RegisterService(&site_service);
    builder.RegisterService(&podman_service);
    builder.RegisterService(&file_system_service);
  }

  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
  spdlog::info("listen a gRPC server on tcp://{}:{}", host, port);

  server->Wait();
}
static void generate_token_for_user(const toml::table& config,
                                    const std::string& username,
                                    uint8_t years) {
  spdlog::info("generate token for user {} with {} years", username, years);
  auto jwt = open_jwt(config);
  const auto token = jwt->sign(palm::CurrentUser::ISSUER, username,
                               {palm::CurrentUser::WEB_AUDIENCE}, std::nullopt,
                               std::chrono::years{years});
  std::cout << token << std::endl;
  spdlog::info("done.");
}

void palm::phlox::Application::launch(int argc, char* argv[]) {
  bool debug;
  std::string config_file;

  std::string http_listen_host;
  int http_listen_port;

  std::string rpc_listen_host;
  int rpc_listen_port;

  std::vector<std::string> watcher_files;
  bool watcher_stdin;

  std::string generate_token_username;
  int generate_token_years;

  std::string generate_etc_domain;

  argparse::ArgumentParser program(
      "phlox", std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME));
  program.add_description("centralize, transform & stash your logging data.");
  program.add_epilog(palm::PROJECT_HOME);
  program.add_argument("-c", "--config")
      .default_value("config.toml")
      .store_into(config_file)
      .help("configuration file");
  program.add_argument("-d", "--debug")
      .flag()
      .store_into(debug)
      .help("run on debug mode");

  argparse::ArgumentParser http_command("http");
  http_command.add_description("start a HTTP server");
  http_command.add_argument("-H", "--host")
      .default_value("127.0.0.1")
      .store_into(http_listen_host);
  http_command.add_argument("-p", "--port")
      .default_value(8080)
      .store_into(http_listen_port);

  argparse::ArgumentParser rpc_command("rpc");
  rpc_command.add_description("start a gRPC server");
  rpc_command.add_argument("-H", "--host")
      .default_value("127.0.0.1")
      .store_into(rpc_listen_host);
  rpc_command.add_argument("-p", "--port")
      .default_value(8080)
      .store_into(rpc_listen_port);

  argparse::ArgumentParser generate_etc_command("generate-etc");
  generate_etc_command.add_description("generate system configuration files");
  generate_etc_command.add_argument("-n", "--domain-name")
      .required()
      .store_into(generate_etc_domain);

  argparse::ArgumentParser generate_token_command("generate-token");
  generate_token_command.add_description("generate a token for user");
  generate_token_command.add_argument("-u", "--username")
      .store_into(generate_token_username)
      .required();
  generate_token_command.add_argument("-y", "--years")
      .default_value(1)
      .store_into(generate_token_years);

  argparse::ArgumentParser podman_logs_command("podman-logs");
  podman_logs_command.add_description("fetch the logs of podman containers");
  argparse::ArgumentParser podman_stats_command("podman-stats");
  podman_stats_command.add_description(
      "podman container resource usage statistics");
  argparse::ArgumentParser podman_ps_command("podman-ps");
  podman_ps_command.add_description("fetch podman containers");

  argparse::ArgumentParser fs_watcher_command("fs-watcher");
  fs_watcher_command.add_description("start a log files watcher");
  fs_watcher_command.add_argument("-f", "--files")
      .append()
      .store_into(watcher_files)
      .help("log files to watch");
  fs_watcher_command.add_argument("-s", "--stdin")
      .flag()
      .store_into(watcher_stdin)
      .help("input from stdin");

  program.add_subparser(http_command);
  program.add_subparser(rpc_command);
  program.add_subparser(generate_etc_command);
  program.add_subparser(generate_token_command);
  program.add_subparser(podman_logs_command);
  program.add_subparser(podman_stats_command);
  program.add_subparser(podman_ps_command);
  program.add_subparser(fs_watcher_command);
  program.parse_args(argc, argv);

  if (program.is_subcommand_used(generate_etc_command)) {
    generate_etc(generate_etc_domain);
    return;
  }

  if (program.is_subcommand_used(http_command) ||
      program.is_subcommand_used(rpc_command) ||
      program.is_subcommand_used(generate_token_command) ||
      program.is_subcommand_used(fs_watcher_command) ||
      program.is_subcommand_used(podman_logs_command) ||
      program.is_subcommand_used(podman_stats_command) ||
      program.is_subcommand_used(podman_ps_command)) {
    palm::init(debug);
    spdlog::info("load configuration from {}", config_file);
    toml::table config = toml::parse_file(config_file);
    if (program.is_subcommand_used(http_command)) {
      start_http_server(http_listen_host, http_listen_port, config);
      return;
    }
    if (program.is_subcommand_used(rpc_command)) {
      start_rpc_server(rpc_listen_host, rpc_listen_port, config);
      return;
    }
    if (program.is_subcommand_used(generate_token_command)) {
      generate_token_for_user(config, generate_token_username,
                              static_cast<uint8_t>(generate_token_years));
      return;
    }
    if (program.is_subcommand_used(fs_watcher_command)) {
      start_log_watcher(config, watcher_stdin, watcher_files);
      return;
    }
    if (program.is_subcommand_used(podman_logs_command)) {
      launch_podman_logs(config);
      return;
    }
    if (program.is_subcommand_used(podman_stats_command)) {
      launch_podman_stats(config);
      return;
    }
    if (program.is_subcommand_used(podman_ps_command)) {
      launch_podman_ps(config);
      return;
    }
  }
  std::cout << program << std::endl;
}
