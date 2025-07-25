#include "palm/podman.hpp"
#include "palm/utils.hpp"

#include <cstdlib>
#include <sstream>

// https://github.com/boostorg/process/blob/boost-1.88.0/example/stdio.cpp

/*
podman stats -a --format json --no-stream
podman ps -a --format json
podman logs --since "2025-07-17T01:00:00" --until "2025-07-18T02:00:00" -n -t
091c84e9e812
*/

// podman info --format '{{ .Host.LogDriver }}'
// systemctl --user list-unit-files --all
// journalctl -u podman --output=json
// journalctl --output=json-pretty -n 20 CONTAINER_NAME=xxx

// https://www.freedesktop.org/software/systemd/man/latest/systemd.time.html#Parsing%20Timestamps
static inline std::string std_tm_to_journald_timestamp(std::tm* t) {
  char buf[80];
  strftime(buf, sizeof(buf), "%Y-%m-%d %H:%M:%S", t);
  return buf;
}
std::vector<palm::podman::models::Log> palm::podman::logs(
    const std::string& container_id, std::tm* since_, std::tm* until_) {
  spdlog::debug("SINCE {} UNTIL {}", std::asctime(since_),
                std::asctime(until_));
  const std::string since = std_tm_to_journald_timestamp(since_);
  const std::string until = std_tm_to_journald_timestamp(until_);
  spdlog::info("fetch logs for {} from {} to {}", container_id, since, until);

  const auto& [code, out, err] = palm::shell(
      "/usr/bin/journalctl",
      {"--user", "--output", "json", "--since", since, "--until", until});
  if (code != EXIT_SUCCESS) {
    spdlog::error("{}", err);
    return {};
  }

  const auto js = nlohmann::json::parse(out);
  std::vector<palm::podman::models::Log> items = js;
  return items;
}

std::vector<palm::podman::models::Status> palm::podman::stats(bool all) {
  std::vector<std::string> args = {"stats", "--format", "json", "--no-stream"};
  if (all) {
    args.push_back("-a");
  }

  const auto& [code, out, err] = palm::shell("/usr/bin/podman", args);
  if (code != EXIT_SUCCESS) {
    spdlog::error("{}", err);
    return {};
  }

  const auto js = nlohmann::json::parse(out);
  std::vector<palm::podman::models::Status> items = js;
  return items;
}
std::vector<palm::podman::models::container::Item> palm::podman::ps(bool all) {
  std::vector<std::string> args = {"ps", "-a", "--format", "json"};
  if (all) {
    args.push_back("-a");
  }

  const auto& [code, out, err] = palm::shell("/usr/bin/podman", args);
  if (code != EXIT_SUCCESS) {
    spdlog::error("{}", err);
    return {};
  }

  const auto js = nlohmann::json::parse(out);
  std::vector<palm::podman::models::container::Item> items = js;
  return items;
}
