#include "basil/hibiscus.hpp"
#include "basil/bbs.hpp"
#include "basil/blog.hpp"
#include "basil/bookkeeper.hpp"
#include "basil/cms.hpp"
#include "basil/questionnaire.hpp"
#include "basil/utils.hpp"
#include "basil/version.hpp"

#include <unistd.h>
#include <iostream>
#include <regex>
#include <stdexcept>

#include <boost/algorithm/string.hpp>
#include <boost/asio/ip/host_name.hpp>
#include <boost/log/core.hpp>
#include <boost/log/expressions.hpp>
#include <boost/optional/optional.hpp>
#include <boost/program_options.hpp>
#include <boost/property_tree/ini_parser.hpp>
#include <boost/range/iterator_range.hpp>

#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/health_check_service_interface.h>
#include <mysql/mariadb_version.h>
#include <openssl/opensslv.h>

static std::shared_ptr<basil::Jwt> open_jwt(
    const boost::property_tree::ptree& config) {
  const auto node = config.get_child("jwt");
  std::shared_ptr<basil::Jwt> it =
      std::make_shared<basil::Jwt>(node.get<std::string>("key"));
  return it;
}
static std::shared_ptr<basil::Aes> open_aes(
    const boost::property_tree::ptree& config) {
  const auto node = config.get_child("aes");
  std::shared_ptr<basil::Aes> it = std::make_shared<basil::Aes>(
      node.get<std::string>("key"), node.get<std::string>("iv"));
  return it;
}
static std::shared_ptr<basil::HMac> open_hmac(
    const boost::property_tree::ptree& config) {
  const auto node = config.get_child("hmac");
  std::shared_ptr<basil::HMac> it =
      std::make_shared<basil::HMac>(node.get<std::string>("key"));
  return it;
}
static std::shared_ptr<basil::opensearch::Client> open_opensearch(
    const boost::property_tree::ptree& config) {
  const auto node = config.get_child("opensearch");
  std::shared_ptr<basil::opensearch::Client> it =
      std::make_shared<basil::opensearch::Client>(node.get<std::string>("host"),
                                                  node.get<uint16_t>("port"));
  {
    const auto res = it->cluster_health();
    BOOST_LOG_TRIVIAL(debug)
        << "cluster " << res->cluster_name << "(" << res->number_of_nodes
        << " nodes) " << res->status;
  }
  return it;
}
static std::shared_ptr<basil::Minio> open_minio(
    const boost::property_tree::ptree& config) {
  const auto node = config.get_child("minio");
  std::shared_ptr<basil::Minio> it = std::make_shared<basil::Minio>(
      node.get<std::string>("base-url"), node.get<std::string>("access-key"),
      node.get<std::string>("secret-key"));
  {
    const auto items = it->list_buckets();
    BOOST_LOG_TRIVIAL(debug) << "total " << items.size() << " buckets";
  }
  return it;
}
static std::shared_ptr<sw::redis::Redis> open_redis(
    const boost::property_tree::ptree& config) {
  const auto node = config.get_child("redis");
  boost::optional<std::string> password =
      node.get_optional<std::string>("password");
  basil::redis::Node cfg(
      node.get<std::string>("host"), node.get<uint16_t>("port"),
      (password ? std::optional<std::string>{password.value()} : std::nullopt),
      node.get<uint8_t>("db"), node.get<size_t>("pool-size"));
  auto it = cfg.open();
  return it;
}

static std::shared_ptr<basil::rabbitmq::Config> open_rabbitmq(
    const boost::property_tree::ptree& config) {
  const auto node = config.get_child("rabbitmq");
  std::shared_ptr<basil::rabbitmq::Config> it =
      std::make_shared<basil::rabbitmq::Config>(
          node.get<std::string>("host"), node.get<uint16_t>("port"),
          node.get<std::string>("user"), node.get<std::string>("password"),
          node.get<std::string>("virtual-host"));
  {
    auto con = it->open();
    con->ping();
  }
  return it;
}
static void db_seed(bool debug, const boost::property_tree::ptree& config) {
  // TODO
}
static void start_sms_send_worker(bool debug, const std::string& name,
                                  const boost::property_tree::ptree& config,
                                  uint interval) {
  const auto twilio_node = config.get_child("twilio");
  std::shared_ptr<basil::Twilio> twilio = std::make_shared<basil::Twilio>(
      twilio_node.get<std::string>("account-sid"),
      twilio_node.get<std::string>("auth-token"));

  const auto queue = open_rabbitmq(config);
  auto cli = queue->open();
  std::shared_ptr<basil::QueueConsumer> consumer =
      std::make_shared<basil::wisteria::workers::SmsSendQueueConsumer>(name,
                                                                       twilio);
  cli->consume(basil::wisteria::workers::SmsSendQueueConsumer::QUEUE, consumer);
}

static void start_email_send_worker(bool debug, const std::string& name,
                                    const boost::property_tree::ptree& config,
                                    uint interval) {
  const auto smtp_node = config.get_child("smtp");
  basil::email::Account from{
      .name = smtp_node.get<std::string>("user-name"),
      .email = smtp_node.get<std::string>("user-email"),
  };
  std::shared_ptr<basil::email::Smtp> smtp =
      std::make_shared<basil::email::Smtp>(
          smtp_node.get<std::string>("host"), smtp_node.get<uint16_t>("port"),
          from, smtp_node.get<std::string>("password"));

  const auto queue = open_rabbitmq(config);
  auto cli = queue->open();
  std::shared_ptr<basil::QueueConsumer> consumer =
      std::make_shared<basil::wisteria::workers::EmailSendQueueConsumer>(name,
                                                                         smtp);
  cli->consume(basil::wisteria::workers::EmailSendQueueConsumer::QUEUE,
               consumer);
}

static void start_http_server(const std::string& host, uint16_t port,
                              bool debug,
                              const boost::property_tree::ptree& config,
                              const std::string& theme_folder) {
  if (basil::is_stopped()) {
    return;
  }

  auto s3 = open_minio(config);
  auto search = open_opensearch(config);
  auto jwt = open_jwt(config);

  const auto rpc_node = config.get_child("rpc");
  basil::GrpcClient rpc = basil::GrpcClient(rpc_node.get<std::string>("host"),
                                            rpc_node.get<uint16_t>("port"));
  BOOST_LOG_TRIVIAL(debug) << "connect to backend tcp://" << rpc.target();

  nlohmann::json global;
  {
    global["version"] = basil::GIT_VERSION;
    global["build_time"] = basil::BUILD_TIME;
  }
  basil::Theme theme(theme_folder, global);

  httplib::Server server;

  if (!server.set_mount_point("/3rd", "./vendors")) {
    BOOST_LOG_TRIVIAL(error) << "failed to mount third-party assets";
    return;
  }
  if (!server.set_mount_point("/assets",
                              std::format("{}/assets", theme_folder))) {
    BOOST_LOG_TRIVIAL(error)
        << "failed to mount assets for theme " << theme_folder;
    return;
  }

  server.set_logger([&](const auto& req, const auto& res) {
    std::stringstream params;
    for (auto const& [k, v] : req.params) {
      params << k << "=" << v << " ";
    }
    BOOST_LOG_TRIVIAL(info) << req.method << " " << res.status << " "
                            << req.path << " " << params.str();
  });

  basil::wisteria::mount(server, rpc, theme, jwt, s3);

  BOOST_LOG_TRIVIAL(info) << "listen a HTTP server on tcp://" << host << ":"
                          << port << " with theme " << theme_folder;
  server.listen(host, port);
}

static void start_rpc_server(const std::string& host, uint16_t port, bool debug,
                             const boost::property_tree::ptree& config) {
  if (basil::is_stopped()) {
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

  basil::wisteria::services::UserServiceImpl wisteria_user_service(
      cache, queue, s3, aes, hmac, jwt);
  basil::wisteria::services::PolicyServiceImpl wisteria_policy_service;
  basil::wisteria::services::SiteServiceImpl wisteria_site_service(search);
  basil::cms::services::PageServiceImpl cms_page_service;
  basil::bbs::services::ForumServiceImpl bbs_forum_service;
  basil::bbs::services::TopicServiceImpl bbs_topic_service;
  basil::bbs::services::PostServiceImpl bbs_post_service;
  basil::bookkeeper::services::BookServiceImpl bookkeeper_book_service;
  basil::questionnaire::services::FormServiceImpl questionnaire_form_service;
  basil::blog::services::PageServiceImpl blog_page_service;
  basil::blog::services::PostServiceImpl blog_post_service;

  grpc::EnableDefaultHealthCheckService(true);
  grpc::reflection::InitProtoReflectionServerBuilderPlugin();

  grpc::ServerBuilder builder;
  builder.AddListeningPort(address, grpc::InsecureServerCredentials());

  {
    builder.RegisterService(&wisteria_user_service);
    builder.RegisterService(&wisteria_policy_service);
    builder.RegisterService(&wisteria_site_service);
    builder.RegisterService(&cms_page_service);
    builder.RegisterService(&bbs_forum_service);
    builder.RegisterService(&bbs_topic_service);
    builder.RegisterService(&bbs_post_service);
    builder.RegisterService(&blog_page_service);
    builder.RegisterService(&blog_post_service);
    builder.RegisterService(&questionnaire_form_service);
    builder.RegisterService(&bookkeeper_book_service);
  }

  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
  BOOST_LOG_TRIVIAL(info) << "listen a gRPC server on tcp://" << host << ":"
                          << port;

  server->Wait();
}

void basil::hibiscus::Application::launch(int argc, char* argv[]) {
  boost::program_options::options_description generic("Generic options");
  generic.add_options()("help,h", "print help message")(
      "debug,d", "run on debug mode")("version,v", "print version info")(
      "config,c",
      boost::program_options::value<std::string>()->default_value("config.ini"),
      "configuration file")(
      "host,H",
      boost::program_options::value<std::string>()->default_value("127.0.0.1"),
      "ip address to listening")(
      "port,p", boost::program_options::value<uint16_t>()->default_value(8080),
      "port to listening")(
      "consumer-name",
      boost::program_options::value<std::string>()->default_value(
          std::format("{}.p{}", boost::asio::ip::host_name(), getpid())),
      "queue consumer's name")(
      "task-interval", boost::program_options::value<uint>()->default_value(1),
      "task interval in seconds")(
      "theme,t",
      boost::program_options::value<std::string>()->default_value("bootstrap"),
      "web-page theme folder");
  boost::program_options::options_description services("Sub-commands");
  services.add_options()("rpc", "launch a gRPC server");
  services.add_options()("http", "launch a HTTP server");
  services.add_options()("db-seed",
                         "loads the data from resources into the database");
  services.add_options()("sms-send-consumer",
                         "launch a sms-send consumer processs");
  services.add_options()("email-send-consumer",
                         "launch an email-send consumer processs");

  boost::program_options::options_description all(basil::PROJECT_DESCRIPTION);
  all.add(generic).add(services);

  boost::program_options::variables_map vm;

  boost::program_options::store(
      boost::program_options::parse_command_line(argc, argv, all), vm);
  if (vm.count("help")) {
    std::cout << all << std::endl;
    return;
  }
  if (vm.count("version")) {
    std::cout << basil::GIT_VERSION << "(" << basil::BUILD_TIME << ")"
              << std::endl;
    return;
  }

  const bool debug = vm.count("debug") > 0;
  boost::log::core::get()->set_filter(
      boost::log::trivial::severity >=
      (debug ? boost::log::trivial::debug : boost::log::trivial::info));
  {
    BOOST_LOG_TRIVIAL(debug)
        << "run on debug mode(" << basil::GIT_VERSION << ")";
    BOOST_LOG_TRIVIAL(debug) << OPENSSL_VERSION_TEXT;
    {
      const auto v = PQlibVersion();
      BOOST_LOG_TRIVIAL(debug) << "PostgreSQL v" << v / (100 * 100) << "."
                               << (v / 100) % 100 << "." << v % (100 * 100);
    }
    BOOST_LOG_TRIVIAL(debug) << "MySQL v" << MARIADB_CLIENT_VERSION_STR;
    BOOST_LOG_TRIVIAL(debug) << "Sqlite v" << SQLITE_VERSION;
    BOOST_LOG_TRIVIAL(debug) << "rabbitmq-c v" << AMQ_VERSION_STRING;
    BOOST_LOG_TRIVIAL(debug) << "hiredis v" << HIREDIS_MAJOR << "."
                             << HIREDIS_MINOR << "." << HIREDIS_PATCH;
    BOOST_LOG_TRIVIAL(debug) << "miniocpp v" << MINIO_CPP_VERSION;
  }

  boost::property_tree::ptree tree;
  {
    const std::string config = vm["config"].as<std::string>();
    BOOST_LOG_TRIVIAL(info) << "load from " << config;
    boost::property_tree::ini_parser::read_ini(config, tree);
  }
  const std::string host = vm["host"].as<std::string>();
  const uint16_t port = vm["port"].as<uint16_t>();
  const std::string consumer_name = vm["consumer-name"].as<std::string>();
  const uint task_interval = vm["task-interval"].as<uint>();
  const std::string theme = vm["theme"].as<std::string>();

  if (vm.count("http")) {
    start_http_server(host, port, debug, tree, theme);
    return;
  }
  if (vm.count("rpc")) {
    start_rpc_server(host, port, debug, tree);
    return;
  }
  if (vm.count("db-seed")) {
    db_seed(debug, tree);
    return;
  }
  if (vm.count("sms-send-consumer")) {
    start_sms_send_worker(debug, consumer_name, tree, task_interval);
    return;
  }
  if (vm.count("email-send-consumer")) {
    start_email_send_worker(debug, consumer_name, tree, task_interval);
    return;
  }

  std::cout << services << std::endl;
}
