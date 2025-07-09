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

static std::shared_ptr<palm::Jwt> open_jwt(toml::table* config) {
  auto node = config->get("jwt")->as_table();
  std::optional<std::string> key = node->get("key")->value<std::string>();
  std::shared_ptr<palm::Jwt> it = std::make_shared<palm::Jwt>(key.value());
  return it;
}
static std::shared_ptr<palm::Aes> open_aes(toml::table* config) {
  auto node = config->get("aes")->as_table();
  // TODO
  std::optional<std::string> key = node->get("key")->value<std::string>();
  std::optional<std::string> iv = node->get("iv")->value<std::string>();
  std::shared_ptr<palm::Aes> it =
      std::make_shared<palm::Aes>(key.value(), iv.value());
  return it;
}
static std::shared_ptr<palm::HMac> open_hmac(toml::table* config) {
  auto node = config->get("hmac")->as_table();
  std::optional<std::string> key = node->get("key")->value<std::string>();
  std::shared_ptr<palm::HMac> it = std::make_shared<palm::HMac>(key.value());
  return it;
}
static std::shared_ptr<soci::connection_pool> open_postgresql(
    toml::table* config) {
  auto node = config->get("postgresql")->as_table();
  palm::PostgreSql cfg(node);
  auto it = cfg.open(node->get("pool-size")->value_or<size_t>(1 << 5));
  {
    soci::session db(*it);
    std::string version;
    db << "SELECT VERSION()", soci::into(version);
    spdlog::debug("{}", version);
  }
  return it;
}
static std::shared_ptr<palm::opensearch::Client> open_opensearch(
    toml::table* config) {
  std::shared_ptr<palm::opensearch::Client> it =
      std::make_shared<palm::opensearch::Client>(
          config->get("opensearch")->as_table());
  {
    const auto res = it->cluster_health();
    spdlog::debug("cluster {}({} nodes) {}", res->cluster_name,
                  res->number_of_nodes, res->status);
  }
  return it;
}
static std::shared_ptr<palm::Minio> open_minio(toml::table* config) {
  std::shared_ptr<palm::Minio> it =
      std::make_shared<palm::Minio>(config->get("minio")->as_table());
  {
    const auto items = it->list_buckets();
    spdlog::debug("total {} buckets", items.size());
  }
  return it;
}
static std::shared_ptr<sw::redis::Redis> open_redis(toml::table* config) {
  palm::redis::Node node(config->get("redis")->as_table());
  auto it = node.open();
  {
    const auto v = it->ping();
    spdlog::debug("PING: {}", v);
  }
  return it;
}

static std::shared_ptr<palm::rabbitmq::Config> open_rabbitmq(
    toml::table* config) {
  std::shared_ptr<palm::rabbitmq::Config> it =
      std::make_shared<palm::rabbitmq::Config>(
          config->get("rabbitmq")->as_table());
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
static void generate_etc(toml::table* config, const std::string& domain) {
  // TODO generate chatting/email/www/assets/s3 nginx files
  // TODO generate api/rpc/minio systemd services
}
static void db_seed(toml::table* config) {
  auto db_pool = open_postgresql(config);
  soci::session db(*db_pool);
  {
    soci::transaction tr(db);
    palm::portal::dao::locales::load(db, "locales");
    palm::iso4217::load(db, "iso4217/list-one.xml");
    tr.commit();
  }
}
static void start_sms_send_worker(bool debug, const std::string& name,
                                  toml::table* config, uint interval) {
  std::shared_ptr<palm::Twilio> twilio = std::make_shared<palm::Twilio>(config);

  const auto queue = open_rabbitmq(config);
  auto cli = queue->open();
  std::shared_ptr<palm::QueueConsumer> consumer =
      std::make_shared<palm::portal::workers::SmsSendQueueConsumer>(name,
                                                                    twilio);
  cli->consume(palm::portal::workers::SmsSendQueueConsumer::QUEUE, consumer);
}

static void start_email_send_worker(bool debug, const std::string& name,
                                    toml::table* config, uint interval) {
  std::shared_ptr<palm::email::Smtp> smtp =
      std::make_shared<palm::email::Smtp>(config);

  const auto queue = open_rabbitmq(config);
  auto cli = queue->open();
  std::shared_ptr<palm::QueueConsumer> consumer =
      std::make_shared<palm::portal::workers::EmailSendQueueConsumer>(name,
                                                                      smtp);
  cli->consume(palm::portal::workers::EmailSendQueueConsumer::QUEUE, consumer);
}

static void start_http_server(const std::string& host, uint16_t port,
                              bool debug, toml::table* config,
                              const std::string& theme_folder) {
  if (palm::is_stopped()) {
    return;
  }

  auto s3 = open_minio(config->get("minio")->as_table());
  auto search = open_opensearch(config->get("opensearch")->as_table());
  auto jwt = open_jwt(config->get("jwt")->as_table());

  palm::GrpcClient rpc = palm::GrpcClient(config->get("backend")->as_table());
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
                             toml::table* config) {
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

  argparse::ArgumentParser program(
      "lavender", std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME));
  program.add_description(palm::PROJECT_DESCRIPTION);
  program.add_epilog(palm::PROJECT_HOME);
  program.add_argument("-c", "--config")
      .default_value("config.toml")
      .store_into(config_file)
      .help("Configuration file");
  program.add_argument("-d", "--debug")
      .flag()
      .store_into(debug)
      .help("Run on debug mode");

  argparse::ArgumentParser http_command("http");
  http_command.add_description("Start a HTTP server");
  http_command.add_argument("-H", "--host")
      .default_value("127.0.0.1")
      .store_into(http_listen_host)
      .help("IP address to listen");
  http_command.add_argument("-p", "--port")
      .default_value(8080)
      .store_into(http_listen_port)
      .help("Port to listen");
  http_command.add_argument("-t", "--theme")
      .default_value("bootstrap")
      .store_into(http_theme_folder)
      .help("Folder to load theme");

  argparse::ArgumentParser rpc_command("rpc");
  rpc_command.add_description("Start a gRPC server");
  rpc_command.add_argument("-H", "--host")
      .default_value("127.0.0.1")
      .store_into(rpc_listen_host)
      .help("IP address to listen");
  rpc_command.add_argument("-p", "--port")
      .default_value(8080)
      .store_into(rpc_listen_port)
      .help("Port to listen");

  argparse::ArgumentParser db_seed_command("db-seed");

  argparse::ArgumentParser sms_send_consumer_command("sms-send-consumer");
  sms_send_consumer_command.add_argument("-i", "--interval")
      .default_value(3)
      .store_into(sms_send_job_interval)
      .help("SMS task interval(s)");

  argparse::ArgumentParser email_send_consumer_command("email-send-consumer");
  email_send_consumer_command.add_argument("-i", "--interval")
      .default_value(3)
      .store_into(email_send_job_interval)
      .help("Email task interval(s)");

  program.add_subparser(http_command);
  program.add_subparser(rpc_command);
  program.add_subparser(db_seed_command);
  program.add_subparser(sms_send_consumer_command);
  program.add_subparser(email_send_consumer_command);
  program.parse_args(argc, argv);

  palm::init(debug);
  if (program.is_subcommand_used(http_command)) {
    toml::table config = toml::parse_file(config_file);
    start_http_server(http_listen_host, http_listen_port, debug, &config,
                      http_theme_folder);
    return;
  }
  if (program.is_subcommand_used(rpc_command)) {
    toml::table config = toml::parse_file(config_file);
    start_rpc_server(rpc_listen_host, rpc_listen_port, debug, &config);
    return;
  }
  if (program.is_subcommand_used(db_seed_command)) {
    toml::table config = toml::parse_file(config_file);
    db_seed(&config);
    return;
  }
  if (program.is_subcommand_used(sms_send_consumer_command)) {
    toml::table config = toml::parse_file(config_file);
    start_sms_send_worker(debug, sms_send_consumer_name, &config,
                          sms_send_job_interval);
    return;
  }
  if (program.is_subcommand_used(email_send_consumer_command)) {
    toml::table config = toml::parse_file(config_file);
    start_email_send_worker(debug, email_send_consumer_name, &config,
                            email_send_job_interval);
    return;
  }
  std::cout << program << std::endl;
}
