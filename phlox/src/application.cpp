#include "palm/application.hpp"
#include "palm/monitor.hpp"
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
#include <boost/property_tree/ini_parser.hpp>
#include <boost/range/iterator_range.hpp>

#include <argparse/argparse.hpp>

#define PALM_CONFIG_KEY_OPENSEARCH "opensearch"

static std::shared_ptr<palm::Jwt> open_jwt(
    const boost::property_tree::ptree& config) {
  const auto node = config.get_child("jwt");
  std::shared_ptr<palm::Jwt> it =
      std::make_shared<palm::Jwt>(node.get<std::string>("key"));
  return it;
}

static std::shared_ptr<palm::opensearch::Client> open_opensearch(
    const boost::property_tree::ptree& config) {
  const auto node = config.get_child("opensearch");
  std::shared_ptr<palm::opensearch::Client> it =
      std::make_shared<palm::opensearch::Client>(node.get<std::string>("host"),
                                                 node.get<uint16_t>("port"));
  {
    const auto res = it->cluster_health();
    BOOST_LOG_TRIVIAL(debug)
        << "cluster " << res->cluster_name << "(" << res->number_of_nodes
        << " nodes) " << res->status;
  }
  return it;
}

static std::shared_ptr<sw::redis::Redis> open_redis(
    const boost::property_tree::ptree& config) {
  const auto node = config.get_child("redis");
  boost::optional<std::string> password =
      node.get_optional<std::string>("password");
  palm::redis::Node cfg(
      node.get<std::string>("host"), node.get<uint16_t>("port"),
      (password ? std::optional<std::string>{password.value()} : std::nullopt),
      node.get<uint8_t>("db"), node.get<size_t>("pool-size"));
  auto it = cfg.open();
  return it;
}

static void start_log_watcher(bool debug, const toml::table& config, bool stdin,
                              const std::vector<std::string>& original_files) {
  if (palm::is_stopped()) {
    return;
  }

  std::shared_ptr<palm::opensearch::Client> search =
      std::make_shared<palm::opensearch::Client>(
          config[PALM_CONFIG_KEY_OPENSEARCH].as_table());
  {
    auto res = search->cluster_health();
    BOOST_LOG_TRIVIAL(debug) << res->cluster_name << " " << res->status;
  }
  if (!search->index_exists<palm::monitor::logging::Item>()) {
    const auto props = palm::monitor::logging::Item::properties();
    search->create_index<palm::monitor::logging::Item>(2, 1, props);
  }
  palm::monitor::LoggingScratcher scratcher;

  if (stdin) {
    BOOST_LOG_TRIVIAL(info) << "listen from STDIN stream";
    std::shared_ptr<palm::monitor::logging::Source> it =
        std::make_shared<palm::monitor::logging::StdinSource>();
    scratcher.register_(it);
  }
  {
    std::shared_ptr<palm::monitor::logging::FilesystemNotify> it =
        std::make_shared<palm::monitor::logging::FilesystemNotify>();
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
                              bool debug, const toml::table& config,
                              const std::string& theme_folder) {
  if (palm::is_stopped()) {
    return;
  }

  nlohmann::json global;
  {
    global["version"] = palm::GIT_VERSION;
    global["build_time"] = palm::BUILD_TIME;
  }
  palm::Theme theme(theme_folder, global);

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

  // palm::monitor::mount(server, theme, jwt, search);

  BOOST_LOG_TRIVIAL(info) << "listen a HTTP server on tcp://" << host << ":"
                          << port << " with theme " << theme_folder;
  server.listen(host, port);
}

void palm::phlox::Application::launch(int argc, char* argv[]) {
  bool debug;
  std::string config_file;
  std::string http_listen_host;
  int http_listen_port;
  std::string http_theme_folder;
  std::vector<std::string> watcher_files;
  bool watcher_stdin;

  argparse::ArgumentParser program("phlox", GIT_VERSION);
  program.add_description("Centralize, transform & stash your logging data.");
  program.add_epilog("https://github.com/saturn-xiv/palm");
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

  argparse::ArgumentParser watcher_command("watcher");
  watcher_command.add_description("Start a log files watcher");
  watcher_command.add_argument("-f", "--files")
      .append()
      .store_into(watcher_files)
      .help("Log files to watch");
  watcher_command.add_argument("-s", "--stdin")
      .flag()
      .store_into(watcher_stdin)
      .help("Input from stdin");

  program.add_subparser(http_command);
  program.add_subparser(watcher_command);
  program.parse_args(argc, argv);

  {
    boost::log::core::get()->set_filter(
        boost::log::trivial::severity >=
        (debug ? boost::log::trivial::debug : boost::log::trivial::info));
    BOOST_LOG_TRIVIAL(debug)
        << "run on debug mode(" << palm::GIT_VERSION << ")";
  }
  BOOST_LOG_TRIVIAL(info) << "load configuration from " << config_file;
  toml::table config = toml::parse_file(config_file);
  if (program.is_subcommand_used(http_command)) {
    start_http_server(http_listen_host, http_listen_port, debug, config,
                      http_theme_folder);
    return;
  }
  if (program.is_subcommand_used(watcher_command)) {
    start_log_watcher(debug, config, watcher_stdin, watcher_files);
    return;
  }
  std::cout << program << std::endl;
}
