#include "phlox/docker.hpp"
#include "palm/utils.hpp"

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
