#include "phlox/application.hpp"
#include "palm/utils.hpp"
#include "palm/version.hpp"
#include "phlox/services.hpp"

#include <unistd.h>
#include <format>
#include <iostream>
#include <regex>
#include <stdexcept>

#include <boost/algorithm/string.hpp>
#include <boost/asio.hpp>
#include <boost/asio/ip/host_name.hpp>
#include <boost/optional/optional.hpp>
#include <boost/range/iterator_range.hpp>

#include <argparse/argparse.hpp>

std::shared_ptr<soci::session> phlox::Application::db(
    const toml::table& config) {
  const std::string db_file =
      config["db-file"].value_or<std::string>("db.sqlite3");
  palm::Sqlite3 cfg(db_file);
  auto db = cfg.open();
  {
    soci::transaction tr(*db);
    // COULDN'T put all sql together
    *db << R"SQL(
CREATE TABLE IF NOT EXISTS podman_container_logs (
  id VARCHAR(127) NOT NULL,
  last_fetched_at BIGINT NOT NULL,
  version INTEGER NOT NULL DEFAULT 0, 
  created_at TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_podman_container_logs_id ON podman_container_logs(id);
)SQL";
    *db << R"SQL(
CREATE TABLE IF NOT EXISTS docker_container_logs (
  id VARCHAR(12) NOT NULL,
  last_fetched_at BIGINT NOT NULL,
  version INTEGER NOT NULL DEFAULT 0, 
  created_at TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_docker_container_logs_id ON docker_container_logs(id);
)SQL";
    *db << R"SQL(
CREATE TABLE IF NOT EXISTS systemd_service_logs (
  name VARCHAR(127) NOT NULL,
  last_fetched_at BIGINT NOT NULL,
  version INTEGER NOT NULL DEFAULT 0, 
  created_at TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_systemd_service_logs_name ON systemd_service_logs(name);
)SQL";
    tr.commit();
  }
  return db;
}

std::shared_ptr<palm::Jwt> phlox::Application::jwt(const toml::table& config) {
  auto cfg = config["jwt"].as_table();
  if (cfg == nullptr) {
    spdlog::error("missing jwt part");
    return nullptr;
  }
  std::shared_ptr<palm::Jwt> it =
      std::make_shared<palm::Jwt>((*cfg)["key"].value<std::string>().value());
  return it;
}

std::shared_ptr<palm::opensearch::Client> phlox::Application::opensearch(
    const toml::table& config) {
  std::shared_ptr<palm::opensearch::Client> it =
      std::make_shared<palm::opensearch::Client>(
          *(config["opensearch"].as_table()));
  {
    const auto res = it->cluster_health();
    spdlog::debug("cluster {} ({} nodes) {}", res->cluster_name,
                  res->number_of_nodes, res->status);
  }
  if (!it->index_exists<
          palm::monitoring::v1::PodmanContainersResponse_Item>()) {
    nlohmann::json props;
    {
      nlohmann::json it;
      it["type"] = "text";
      props["host"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["id"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["name"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["updatedAt"] = it;
    }

    it->create_index<palm::monitoring::v1::PodmanContainersResponse_Item>(
        2, 1, props);
  }
  if (!it->index_exists<palm::monitoring::v1::PodmanLogsResponse_Item>()) {
    nlohmann::json props;

    {
      nlohmann::json it;
      it["type"] = "text";
      props["host"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["id"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["full_id"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["name"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["message"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["createdAt"] = it;
    }

    it->create_index<palm::monitoring::v1::PodmanLogsResponse_Item>(2, 1,
                                                                    props);
  }
  if (!it->index_exists<
          palm::monitoring::v1::PodmanStatisticsResponse_Item>()) {
    nlohmann::json props;
    {
      nlohmann::json it;
      it["type"] = "text";
      props["host"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["id"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["name"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["createdAt"] = it;
    }

    it->create_index<palm::monitoring::v1::PodmanStatisticsResponse_Item>(
        2, 1, props);
  }
  if (!it->index_exists<
          palm::monitoring::v1::DockerStatisticsResponse_Item>()) {
    nlohmann::json props;
    {
      nlohmann::json it;
      it["type"] = "text";
      props["host"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["id"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["name"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["createdAt"] = it;
    }

    it->create_index<palm::monitoring::v1::DockerStatisticsResponse_Item>(
        2, 1, props);
  }
  if (!it->index_exists<
          palm::monitoring::v1::DockerContainersResponse_Item>()) {
    nlohmann::json props;
    {
      nlohmann::json it;
      it["type"] = "text";
      props["host"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["id"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["name"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["createdAt"] = it;
    }

    it->create_index<palm::monitoring::v1::DockerContainersResponse_Item>(
        2, 1, props);
  }
  if (!it->index_exists<palm::monitoring::v1::FileSystemLogsResponse_Item>()) {
    nlohmann::json props;

    {
      nlohmann::json it;
      it["type"] = "text";
      props["host"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["file"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["message"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["createdAt"] = it;
    }
    const auto val =
        it->create_index<palm::monitoring::v1::FileSystemLogsResponse_Item>(
            2, 1, props);
  }
  if (!it->index_exists<palm::monitoring::v1::SystemdJournalResponse_Item>()) {
    nlohmann::json props;

    {
      nlohmann::json it;
      it["type"] = "text";
      props["host"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["name"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["message"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["createdAt"] = it;
    }

    it->create_index<palm::monitoring::v1::SystemdJournalResponse_Item>(2, 1,
                                                                        props);
  }
  return it;
}

void phlox::Application::launch(int argc, char* argv[]) {
  bool debug;
  std::string config_file;

  std::string http_listen_host;
  int http_listen_port;

  std::string rpc_listen_host;
  int rpc_listen_port;

  std::vector<std::string> fs_watcher_files;
  bool fs_watcher_stdin;

  std::string generate_token_username;
  int generate_token_years;

  std::string generate_etc_domain;

  bool systemd_journal_user_scope;
  std::string systemd_journal_service_name;

  bool podman_stats_all;
  bool podman_ps_all;

  bool docker_stats_all;
  bool docker_ps_all;

  argparse::ArgumentParser program(
      "phlox", std::format("{}({})", palm::GIT_VERSION, palm::BUILD_TIME));
  program.add_description("Centralize, transform & stash your logging data.");
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
  podman_logs_command.add_description(
      "fetch the logs of podman containers' logs");

  argparse::ArgumentParser podman_stats_command("podman-stats");
  podman_stats_command.add_description(
      "podman container resource usage statistics");
  podman_stats_command.add_argument("-a", "--all")
      .flag()
      .store_into(podman_stats_all);

  argparse::ArgumentParser podman_ps_command("podman-ps");
  podman_ps_command.add_description("fetch podman containers");
  podman_ps_command.add_argument("-a", "--all")
      .flag()
      .store_into(podman_ps_all);

  argparse::ArgumentParser docker_logs_command("docker-logs");
  docker_logs_command.add_description(
      "fetch the logs of docker containers' logs");
  argparse::ArgumentParser docker_ps_command("docker-ps");
  docker_ps_command.add_description("fetch docker containers");
  docker_ps_command.add_argument("-a", "--all")
      .flag()
      .store_into(docker_ps_all);

  argparse::ArgumentParser docker_stats_command("docker-stats");
  docker_stats_command.add_description(
      "docker container resource usage statistics");
  docker_stats_command.add_argument("-a", "--all")
      .flag()
      .store_into(docker_stats_all);

  argparse::ArgumentParser systemd_journal_command("systemd-journal");
  systemd_journal_command.add_description("fetch systemd service logs");
  systemd_journal_command.add_argument("-u", "--user-scope")
      .flag()
      .store_into(systemd_journal_user_scope)
      .help("run as user scope");
  systemd_journal_command.add_argument("-s", "--service")
      .store_into(systemd_journal_service_name)
      .required();

  argparse::ArgumentParser fs_watcher_command("fs-watcher");
  fs_watcher_command.add_description("start a log files watcher");
  fs_watcher_command.add_argument("-f", "--files")
      .append()
      .store_into(fs_watcher_files)
      .help("log files to watch");
  fs_watcher_command.add_argument("-s", "--stdin")
      .flag()
      .store_into(fs_watcher_stdin)
      .help("input from stdin");

  program.add_subparser(http_command);
  program.add_subparser(rpc_command);
  program.add_subparser(generate_etc_command);
  program.add_subparser(generate_token_command);
  program.add_subparser(podman_logs_command);
  program.add_subparser(podman_stats_command);
  program.add_subparser(podman_ps_command);
  program.add_subparser(docker_logs_command);
  program.add_subparser(docker_stats_command);
  program.add_subparser(docker_ps_command);
  program.add_subparser(systemd_journal_command);
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
      program.is_subcommand_used(systemd_journal_command) ||
      program.is_subcommand_used(podman_logs_command) ||
      program.is_subcommand_used(podman_stats_command) ||
      program.is_subcommand_used(podman_ps_command) ||
      program.is_subcommand_used(docker_logs_command) ||
      program.is_subcommand_used(docker_stats_command) ||
      program.is_subcommand_used(docker_ps_command)) {
    palm::init(debug);
    spdlog::info("load configuration from {}", config_file);
    toml::table config = toml::parse_file(config_file);
    if (program.is_subcommand_used(http_command)) {
      this->http_server(config, http_listen_host, http_listen_port);
      return;
    }
    if (program.is_subcommand_used(rpc_command)) {
      this->rpc_server(config, rpc_listen_host, rpc_listen_port);
      return;
    }
    if (program.is_subcommand_used(generate_token_command)) {
      this->generate_token(config, generate_token_username,
                           static_cast<uint8_t>(generate_token_years));
      return;
    }
    if (program.is_subcommand_used(fs_watcher_command)) {
      this->fs_watcher(config, fs_watcher_stdin, fs_watcher_files);
      return;
    }
    if (program.is_subcommand_used(podman_logs_command)) {
      this->podman_logs(config);
      return;
    }
    if (program.is_subcommand_used(podman_stats_command)) {
      this->podman_stats(config, podman_stats_all);
      return;
    }
    if (program.is_subcommand_used(podman_ps_command)) {
      this->podman_ps(config, podman_ps_all);
      return;
    }
    if (program.is_subcommand_used(docker_logs_command)) {
      this->docker_logs(config);
      return;
    }
    if (program.is_subcommand_used(docker_stats_command)) {
      this->docker_stats(config, docker_stats_all);
      return;
    }
    if (program.is_subcommand_used(docker_ps_command)) {
      this->docker_ps(config, docker_ps_all);
      return;
    }
    if (program.is_subcommand_used(systemd_journal_command)) {
      this->systemd_journal(config, systemd_journal_service_name,
                            systemd_journal_user_scope);
      return;
    }
  }
  std::cout << program << std::endl;
}
