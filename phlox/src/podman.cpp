#include "phlox/podman.hpp"
#include "palm/utils.hpp"
#include "phlox/systemd.hpp"

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

std::vector<phlox::podman::models::Log> phlox::podman::logs(
    const std::string& container_id, time_t since_, time_t until_) {
  const std::string since = phlox::epoch_to_journald_timestamp(since_);
  const std::string until = phlox::epoch_to_journald_timestamp(until_);
  spdlog::info("fetch logs for {} from {} to {}", container_id, since, until);

  const auto& [code, out, err] =
      palm::shell("/usr/bin/journalctl",
                  {"--user", "--output", "json", "--since", since, "--until",
                   until, std::format("CONTAINER_ID_FULL={}", container_id)});
  if (code != EXIT_SUCCESS) {
    spdlog::error("{}", err);
    return {};
  }

  std::vector<std::string> lines;
  std::vector<phlox::podman::models::Log> items;
  boost::split(lines, out, boost::is_any_of("\n"));
  for (auto& line : lines) {
    boost::trim(line);
    if (line.empty()) {
      continue;
    }
    spdlog::debug("receive podman log: {}", line);
    const auto js = nlohmann::json::parse(line);
    const auto it = js.template get<phlox::podman::models::Log>();
    items.push_back(it);
  }

  return items;
}

std::vector<phlox::podman::models::Status> phlox::podman::stats(bool all) {
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
  std::vector<phlox::podman::models::Status> items = js;
  return items;
}
std::vector<phlox::podman::models::container::Item> phlox::podman::ps(
    bool all) {
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
  std::vector<phlox::podman::models::container::Item> items = js;
  return items;
}
