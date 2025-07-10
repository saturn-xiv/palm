#include "palm/application.hpp"
#include "palm/monitoring.hpp"
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

#include <argparse/argparse.hpp>

static std::shared_ptr<palm::Jwt> open_jwt(const toml::table& config) {
  std::shared_ptr<palm::Jwt> it =
      std::make_shared<palm::Jwt>(config["key"].value<std::string>().value());
  return it;
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

static std::shared_ptr<sw::redis::Redis> open_redis(const toml::table& config) {
  palm::redis::Node node(*(config["redis"].as_table()));
  auto it = node.open();
  {
    const auto v = it->ping();
    spdlog::debug("PING: {}", v);
  }
  return it;
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
                              const toml::table& config,
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
    spdlog::error("failed to mount third-party assets");
    return;
  }
  if (!server.set_mount_point("/assets",
                              std::format("{}/assets", theme_folder))) {
    spdlog::error("failed to mount assets for theme {}", theme_folder);
    return;
  }

  palm::set_logger(server);

  // palm::monitor::mount(server, theme, jwt, search);

  spdlog::info("listen a HTTP server on tcp://{}:{} with theme {}", host, port,
               theme_folder);
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

  argparse::ArgumentParser program(
      "phlox", std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME));
  program.add_description("Centralize, transform & stash your logging data.");
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

  if (program.is_subcommand_used(http_command) ||
      program.is_subcommand_used(watcher_command)) {
    palm::init(debug);
    spdlog::info("load configuration from {}", config_file);
    toml::table config = toml::parse_file(config_file);
    if (program.is_subcommand_used(http_command)) {
      start_http_server(http_listen_host, http_listen_port, config,
                        http_theme_folder);
      return;
    }
    if (program.is_subcommand_used(watcher_command)) {
      start_log_watcher(config, watcher_stdin, watcher_files);
      return;
    }
  }
  std::cout << program << std::endl;
}
