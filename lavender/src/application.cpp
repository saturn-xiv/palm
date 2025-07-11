#include "palm/application.hpp"
#include "palm/babel.hpp"
#include "palm/bbs.hpp"
#include "palm/blog.hpp"
#include "palm/cms.hpp"
#include "palm/iso4217.hpp"
#include "palm/ledger.hpp"
#include "palm/portal.hpp"
#include "palm/survey.hpp"
#include "palm/utils.hpp"
#include "palm/version.hpp"

#include <unistd.h>
#include <iostream>
#include <regex>
#include <stdexcept>

#include <boost/algorithm/string.hpp>
#include <boost/asio/ip/host_name.hpp>
#include <boost/optional/optional.hpp>
#include <boost/range/iterator_range.hpp>

// TODO
// #include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/health_check_service_interface.h>
#include <argparse/argparse.hpp>

static std::shared_ptr<palm::Jwt> open_jwt(const toml::table& config) {
  auto node = config["jwt"].as_table();
  std::optional<std::string> key = node->get("key")->value<std::string>();
  std::shared_ptr<palm::Jwt> it = std::make_shared<palm::Jwt>(key.value());
  return it;
}
static std::shared_ptr<palm::Aes> open_aes(const toml::table& config) {
  auto node = config["aes"].as_table();
  // TODO
  std::optional<std::string> key = node->get("key")->value<std::string>();
  std::optional<std::string> iv = node->get("iv")->value<std::string>();
  std::shared_ptr<palm::Aes> it =
      std::make_shared<palm::Aes>(key.value(), iv.value());
  return it;
}
static std::shared_ptr<palm::HMac> open_hmac(const toml::table& config) {
  auto node = config["hmac"].as_table();
  std::optional<std::string> key = node->get("key")->value<std::string>();
  std::shared_ptr<palm::HMac> it = std::make_shared<palm::HMac>(key.value());
  return it;
}
static std::shared_ptr<soci::connection_pool> open_postgresql(
    const toml::table& config) {
  palm::PostgreSql cfg(*(config["postgresql"].as_table()));
  auto it = cfg.open();
  {
    soci::session db(*it);
    std::string version;
    db << "SELECT VERSION()", soci::into(version);
    spdlog::debug("{}", version);
  }
  return it;
}
static std::shared_ptr<palm::opensearch::Client> open_opensearch(
    const toml::table& config) {
  auto cfg = config["opensearch"].as_table();
  std::shared_ptr<palm::opensearch::Client> it =
      std::make_shared<palm::opensearch::Client>(*cfg);
  {
    const auto res = it->cluster_health();
    spdlog::debug("cluster {}({} nodes) {}", res->cluster_name,
                  res->number_of_nodes, res->status);
  }
  return it;
}
static std::shared_ptr<palm::Minio> open_minio(const toml::table& config) {
  auto cfg = config["minio"].as_table();
  std::shared_ptr<palm::Minio> it = std::make_shared<palm::Minio>(*cfg);
  {
    const auto items = it->list_buckets();
    spdlog::debug("total {} buckets", items.size());
  }
  return it;
}
static std::shared_ptr<sw::redis::Redis> open_redis(const toml::table& config) {
  auto cfg = config["redis"].as_table();
  palm::redis::Node node(*cfg);
  auto it = node.open();
  {
    const auto v = it->ping();
    spdlog::debug("PING: {}", v);
  }
  return it;
}

static std::shared_ptr<palm::rabbitmq::Config> open_rabbitmq(
    const toml::table& config) {
  auto cfg = config["rabbitmq"].as_table();
  std::shared_ptr<palm::rabbitmq::Config> it =
      std::make_shared<palm::rabbitmq::Config>(*cfg);
  {
    auto con = it->open();
    con->ping();
  }
  return it;
}
static void create_email_user(toml::table* config, const std::string& real_name,
                              const std::string& email,
                              const std::string& password) {
  // TODO
}
static void set_email_user_password(toml::table* config,
                                    const std::string& email,
                                    const std::string& password) {
  // TODO
}
static void add_role_for_email_user(toml::table* config,
                                    const std::string& email,
                                    const std::string& role) {
  // TODO
}
static void delete_role_for_email_user(toml::table* config,
                                       const std::string& email,
                                       const std::string& role) {
  // TODO
}
static void generate_etc(const toml::table& config, const std::string& domain) {
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
    std::vector<std::string> prefixes = {"chatting", "mail", "www"};
    nlohmann::json data = {
        {"domain", domain},
        {"prefixes", {"chatting", "mail", "www"}},
        {"minio",
         {{"hosts", {"192.168.21", "192.168.22", "192.168.23"}},
          {"port", 9000},
          {"console_port", 9001}}},
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
## for prefix in prefixes
server {
  listen 80;

  server_name {{ prefix }}.{{ domain }};
  access_log /var/log/nginx/{{ prefix }}.{{ domain }}.access.log;
  error_log  /var/log/nginx/{{ prefix }}.{{ domain }}.error.log;

  gzip on;
  gzip_comp_level 9;
  gzip_min_length 1k;
  gzip_types text/plain text/css application/xml application/javascript;
  gzip_vary on;
  client_max_body_size 128M;

  location /my/ {
    alias /usr/local/share/lavender/{{ prefix }}/dashboard/;
    try_files $uri $uri/ /my/index.html;

    location ~* \.(css|js|png|jpg|jpeg|gif|gz|svg|mp4|ogg|ogv|webm|htc|xml|woff)$ {
      access_log off;
      expires max;
    }
  }
  location /3rd/ {
    alias /user/local/share/lavender/node_modules/;

    location ~* \.(css|js|png|jpg|jpeg|gif|gz|svg|mp4|ogg|ogv|webm|htc|xml|woff)$ {
      access_log off;
      expires max;
    }
  }
  location /assets/ {
    alias /user/local/share/{{ domain }}/assets/;

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
## endfor

# -----------------------------------------------------------------------------

# https://min.io/docs/minio/linux/integrations/setup-nginx-proxy-with-minio.html
upstream assets_{{ domain }} {
  least_conn;
## for host in minio.hosts
  server {{ host }}:{{ minio.port }};
## endfor
}

upstream s3_{{ domain }} {
  least_conn;
## for host in minio.hosts
  server {{ host }}:{{ minio.console_port }};
## endfor
}

server {
  listen 80;   
  server_name assets.{{ domain }};
  access_log /var/log/nginx/assets.{{ domain }}.access.log;
  error_log  /var/log/nginx/assets.{{ domain }}.error.log;

  # Allow special characters in headers
  ignore_invalid_headers off;
  # Allow any size file to be uploaded.
  # Set to a value such as 1000m; to restrict file size to a specific value
  client_max_body_size 0;
  # Disable buffering
  proxy_buffering off;
  proxy_request_buffering off;

  location / {
    proxy_set_header Host $http_host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;

    proxy_connect_timeout 300;
    # Default is HTTP/1, keepalive is only enabled in HTTP/1.1
    proxy_http_version 1.1;
    proxy_set_header Connection "";
    chunked_transfer_encoding off;

    proxy_pass http://assets_{{ domain }};
  }
}

server {
  listen 80;
  server_name  s3.{{ domain }};
  access_log /var/log/nginx/s3.{{ domain }}.access.log;
  error_log  /var/log/nginx/s3.{{ domain }}.error.log;

  # Allow special characters in headers
  ignore_invalid_headers off;
  # Allow any size file to be uploaded.
  # Set to a value such as 1000m; to restrict file size to a specific value
  client_max_body_size 0;
  # Disable buffering
  proxy_buffering off;
  proxy_request_buffering off;

  location / {
    proxy_set_header Host $http_host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_set_header X-NginX-Proxy true;

    # This is necessary to pass the correct IP to be hashed
    real_ip_header X-Real-IP;

    proxy_connect_timeout 300;

    # To support websocket
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";

    chunked_transfer_encoding off;

    proxy_pass http://s3_{{ domain }}; 
  }
}

# -----------------------------------------------------------------------------

server {
  listen 80;
  root /var/lib/ftp.{{ domain }};
  index index.html;
  server_name ftp.{{ domain }};
  access_log /var/log/nginx/ftp.{{ domain }}.access.log;
  error_log  /var/log/nginx/ftp.{{ domain }}.error.log;
  charset utf-8;

  location / {
    autoindex on;
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
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/var/lib/{{ domain }}
ExecStart=/usr/bin/local/lavender -c http.toml -p {{ port }}
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
ExecStart=/usr/local/bin/lavender -c rpc.toml -p {{ port }}
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
    const std::vector<std::string> consumers = {"sms-send", "email-send"};
    for (const auto& name : consumers) {
      const auto file = etc / std::format("{}.consumer.{}.conf", name, domain);
      spdlog::info("generate file {}", file.string());

      nlohmann::json data = {{"domain", domain}, {"name", name}};

      spdlog::debug("args:\n{}", data.dump(4));
      const std::string tpl = R"SYSTEMD(
[Unit]
Description={{ name }} consumer worker for {{ domain }}
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/var/lib/{{ domain }}
ExecStart=/usr/bin/local/lavender -c {{ name }}-consumer -i 3
# or always, on-abort, on-failure, etc
Restart=always
RestartSec=10s

[Install]
WantedBy=multi-user.target
)SYSTEMD";
      std::ofstream out(file);
      inja::render_to(out, tpl, data);
    }
  }

  // https://min.io/docs/minio/linux/operations/installation.html
  {
    const auto file = etc / std::format("s3.{}.conf", domain);
    spdlog::info("generate file {}", file.string());

    const std::string password = palm::random::alphanumeric(32);
    nlohmann::json data = {{"domain", domain},
                           {"port", 9000},
                           {"console_port", 9001},
                           {"user", "root"},
                           {"password", password}};

    spdlog::debug("args:\n{}", data.dump(4));
    const std::string tpl = R"SYSTEMD(
[Unit]
Description=minio service for {{ domain }}
Documentation=https://min.io/docs/minio/linux/index.html
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/var/lib/s3.{{ domain }}
ExecStart=/usr/local/bin/minio server --address :{{ port }} --console-address :{{ console_port }} data
# or always, on-abort, on-failure, etc
Restart=always

# Specifies the maximum file descriptor number that can be opened by this process
LimitNOFILE=65536

# Specifies the maximum number of threads this process can create
TasksMax=infinity

# Disable timeout logic and wait until process is stopped
TimeoutStopSec=infinity
SendSIGKILL=no

Environment="MINIO_ROOT_USER={{ user }}"
Environment="MINIO_ROOT_PASSWORD={{ password }}"

[Install]
WantedBy=multi-user.target
)SYSTEMD";
    std::ofstream out(file);
    inja::render_to(out, tpl, data);
  }
  spdlog::info("done.");
}
static void db_seed(const toml::table& config) {
  auto db_pool = open_postgresql(config);
  {
    soci::session db(*db_pool);
    {
      soci::transaction tr(db);
      palm::portal::dao::locales::load(db, "locales");
      palm::iso4217::load(db, "iso4217/list-one.xml");
      tr.commit();
    }
  }
  spdlog::info("done.");
}
static void start_sms_send_worker(const std::string& name,
                                  const toml::table& config, uint interval) {
  auto twilio_config = config["twilio"].as_table();
  std::shared_ptr<palm::Twilio> twilio =
      std::make_shared<palm::Twilio>(*twilio_config);

  const auto queue = open_rabbitmq(config);
  auto cli = queue->open();
  std::shared_ptr<palm::QueueConsumer> consumer =
      std::make_shared<palm::portal::workers::SmsSendQueueConsumer>(name,
                                                                    twilio);
  cli->consume(palm::portal::workers::SmsSendQueueConsumer::QUEUE, consumer);
}

static void start_email_send_worker(const std::string& name,
                                    const toml::table& config, uint interval) {
  auto smtp_config = config["smtp"].as_table();
  std::shared_ptr<palm::email::Smtp> smtp =
      std::make_shared<palm::email::Smtp>(*smtp_config);

  const auto queue = open_rabbitmq(config);
  auto cli = queue->open();
  std::shared_ptr<palm::QueueConsumer> consumer =
      std::make_shared<palm::portal::workers::EmailSendQueueConsumer>(name,
                                                                      smtp);
  cli->consume(palm::portal::workers::EmailSendQueueConsumer::QUEUE, consumer);
}

static void start_http_server(const std::string& host, uint16_t port,
                              bool debug, const toml::table& config,
                              const std::string& theme_folder) {
  if (palm::is_stopped()) {
    return;
  }

  auto s3 = open_minio(config);
  auto search = open_opensearch(config);
  auto jwt = open_jwt(config);

  palm::GrpcClient rpc = palm::GrpcClient(*(config["backend"].as_table()));
  spdlog::debug("connect to backend tcp://{}", rpc.target());

  nlohmann::json global;
  {
    global["version"] = palm::GIT_VERSION;
    global["build_time"] = palm::BUILD_TIME;
  }
  palm::Theme theme(theme_folder, global);

  httplib::Server server;

  if (!server.set_mount_point("/3rd", "./vendors")) {
    spdlog::error("failed to mount third-party assets");
    return;
  }
  if (!server.set_mount_point("/assets",
                              std::format("{}/assets", theme_folder))) {
    spdlog::debug("failed to mount assets for theme {}", theme_folder);
    return;
  }

  palm::set_logger(server);
  palm::portal::mount(server, rpc, theme, jwt, s3);

  spdlog::info("listen a HTTP server on tcp://{}:{} with theme {}", host, port,
               theme_folder);
  server.listen(host, port);
}

static void start_rpc_server(const std::string& host, uint16_t port, bool debug,
                             const toml::table& config) {
  if (palm::is_stopped()) {
    return;
  }

  const std::string address = std::format("{}:{}", host, port);
  auto queue = open_rabbitmq(config);
  auto cache = open_redis(config);
  auto s3 = open_minio(config);
  auto search = open_opensearch(config);
  auto jwt = open_jwt(config);
  auto hmac = open_hmac(config);
  auto aes = open_aes(config);

  palm::portal::services::UserServiceImpl portal_user_service(cache, queue, s3,
                                                              aes, hmac, jwt);
  palm::portal::services::PolicyServiceImpl portal_policy_service;
  palm::portal::services::SiteServiceImpl portal_site_service(search);
  palm::cms::services::PageServiceImpl cms_page_service;
  palm::bbs::services::ForumServiceImpl bbs_forum_service;
  palm::bbs::services::TopicServiceImpl bbs_topic_service;
  palm::bbs::services::PostServiceImpl bbs_post_service;
  palm::ledger::services::BookServiceImpl ledger_book_service;
  palm::survey::services::FormServiceImpl survey_form_service;
  palm::blog::services::PageServiceImpl blog_page_service;
  palm::blog::services::PostServiceImpl blog_post_service;

  grpc::EnableDefaultHealthCheckService(true);
  // TODO
  // grpc::reflection::InitProtoReflectionServerBuilderPlugin();

  grpc::ServerBuilder builder;
  builder.AddListeningPort(address, grpc::InsecureServerCredentials());

  {
    builder.RegisterService(&portal_user_service);
    builder.RegisterService(&portal_policy_service);
    builder.RegisterService(&portal_site_service);
    builder.RegisterService(&cms_page_service);
    builder.RegisterService(&bbs_forum_service);
    builder.RegisterService(&bbs_topic_service);
    builder.RegisterService(&bbs_post_service);
    builder.RegisterService(&blog_page_service);
    builder.RegisterService(&blog_post_service);
    builder.RegisterService(&survey_form_service);
    builder.RegisterService(&ledger_book_service);
  }

  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
  spdlog::info("listen a gRPC server on tcp://{}:{}", host, port);

  server->Wait();
}

void palm::lavender::Application::launch(int argc, char* argv[]) {
  bool debug;
  std::string config_file;
  std::string http_listen_host;
  int http_listen_port;
  std::string http_theme_folder;

  std::string rpc_listen_host;
  int rpc_listen_port;

  int sms_send_job_interval;
  std::string sms_send_consumer_name;

  int email_send_job_interval;
  std::string email_send_consumer_name;

  std::string generate_etc_domain;

  argparse::ArgumentParser program(
      "lavender", std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME));
  program.add_description(palm::PROJECT_DESCRIPTION);
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
      .store_into(http_listen_host)
      .help("IP address to listen");
  http_command.add_argument("-p", "--port")
      .default_value(8080)
      .store_into(http_listen_port)
      .help("port to listen");
  http_command.add_argument("-t", "--theme")
      .default_value("bootstrap")
      .store_into(http_theme_folder)
      .help("folder to load theme");

  argparse::ArgumentParser rpc_command("rpc");
  rpc_command.add_description("start a gRPC server");
  rpc_command.add_argument("-H", "--host")
      .default_value("127.0.0.1")
      .store_into(rpc_listen_host)
      .help("IP address to listen");
  rpc_command.add_argument("-p", "--port")
      .default_value(8080)
      .store_into(rpc_listen_port)
      .help("port to listen");

  argparse::ArgumentParser db_seed_command("db-seed");
  db_seed_command.add_description("initialize with the seed data");

  argparse::ArgumentParser generate_etc_command("generate-etc");
  generate_etc_command.add_description("generate system configuration files");
  generate_etc_command.add_argument("-n", "--domain-name")
      .required()
      .store_into(generate_etc_domain);

  argparse::ArgumentParser sms_send_consumer_command("sms-send-consumer");
  sms_send_consumer_command.add_description("start a sms-send consumer worker");
  sms_send_consumer_command.add_argument("-i", "--interval")
      .default_value(3)
      .store_into(sms_send_job_interval)
      .help("SMS task interval(s)");

  argparse::ArgumentParser email_send_consumer_command("email-send-consumer");
  email_send_consumer_command.add_description(
      "start an email-send consumer worker");
  email_send_consumer_command.add_argument("-i", "--interval")
      .default_value(3)
      .store_into(email_send_job_interval)
      .help("email task interval(s)");

  program.add_subparser(http_command);
  program.add_subparser(rpc_command);
  program.add_subparser(db_seed_command);
  program.add_subparser(generate_etc_command);
  program.add_subparser(sms_send_consumer_command);
  program.add_subparser(email_send_consumer_command);
  program.parse_args(argc, argv);

  if (program.is_subcommand_used(http_command) ||
      program.is_subcommand_used(rpc_command) ||
      program.is_subcommand_used(db_seed_command) ||
      program.is_subcommand_used(generate_etc_command) ||
      program.is_subcommand_used(sms_send_consumer_command) ||
      program.is_subcommand_used(email_send_consumer_command)) {
    palm::init(debug);
    spdlog::debug("load configuration from {}", config_file);
    toml::table config = toml::parse_file(config_file);

    if (program.is_subcommand_used(http_command)) {
      start_http_server(http_listen_host, http_listen_port, debug, config,
                        http_theme_folder);
      return;
    }
    if (program.is_subcommand_used(rpc_command)) {
      start_rpc_server(rpc_listen_host, rpc_listen_port, debug, config);
      return;
    }
    if (program.is_subcommand_used(db_seed_command)) {
      db_seed(config);
      return;
    }
    if (program.is_subcommand_used(generate_etc_command)) {
      generate_etc(config, generate_etc_domain);
      return;
    }
    if (program.is_subcommand_used(sms_send_consumer_command)) {
      toml::table config = toml::parse_file(config_file);
      start_sms_send_worker(sms_send_consumer_name, config,
                            sms_send_job_interval);
      return;
    }
    if (program.is_subcommand_used(email_send_consumer_command)) {
      start_email_send_worker(email_send_consumer_name, config,
                              email_send_job_interval);
      return;
    }
  }
  std::cout << program << std::endl;
}
