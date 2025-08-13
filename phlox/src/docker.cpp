#include "phlox/docker.hpp"
#include "palm/utils.hpp"

// docker run --log-driver=journald
// sudo journalctl --output=json -n 20 CONTAINER_ID=9ba72b6fe47a

std::vector<phlox::podman::models::Log> phlox::docker::logs(
    const std::string& container_id, time_t since_, time_t until_) {
  if (!palm::is_root()) {
    spdlog::error("must have root privileges");
    return {};
  }
  const std::string since = phlox::epoch_to_journald_timestamp(since_);
  const std::string until = phlox::epoch_to_journald_timestamp(until_);
  spdlog::info("fetch logs for {} from {} to {}", container_id, since, until);

  const auto& [code, out, err] =
      palm::shell("/usr/bin/journalctl",
                  {"--output", "json", "--since", since, "--until", until,
                   std::format("CONTAINER_ID={}", container_id)});
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
    spdlog::debug("receive docker log: {}", line);
    const auto js = nlohmann::json::parse(line);
    const auto it = js.template get<phlox::podman::models::Log>();
    items.push_back(it);
  }

  return items;
}

std::vector<phlox::docker::models::Status> phlox::docker::stats(bool all) {
  std::vector<std::string> args = {"stats", "--format", "json", "--no-stream"};
  if (all) {
    args.push_back("-a");
  }

  const auto& [code, out, err] = palm::shell("/usr/bin/docker", args);
  if (code != EXIT_SUCCESS) {
    spdlog::error("{}", err);
    return {};
  }

  std::vector<phlox::docker::models::Status> items;
  {
    std::vector<std::string> lines;
    boost::algorithm::split(lines, out, boost::is_any_of("\n"));
    for (std::string& line : lines) {
      boost::algorithm::trim(line);
      if (line.empty()) {
        continue;
      }
      const auto js = nlohmann::json::parse(line);
      auto it = js.template get<phlox::docker::models::Status>();
      items.push_back(it);
    }
  }
  return items;
}
std::vector<phlox::docker::models::Container> phlox::docker::ps(bool all) {
  std::vector<std::string> args = {"ps", "--format", "json"};
  if (all) {
    args.push_back("-a");
  }

  const auto& [code, out, err] = palm::shell("/usr/bin/docker", args);
  if (code != EXIT_SUCCESS) {
    spdlog::error("{}", err);
    return {};
  }

  std::vector<phlox::docker::models::Container> items;
  {
    std::vector<std::string> lines;
    boost::algorithm::split(lines, out, boost::is_any_of("\n"));
    for (std::string& line : lines) {
      boost::algorithm::trim(line);
      if (line.empty()) {
        continue;
      }
      const auto js = nlohmann::json::parse(line);
      auto it = js.template get<phlox::docker::models::Container>();
      items.push_back(it);
    }
  }
  return items;
}
