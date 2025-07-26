#include "palm/systemd.hpp"
#include "palm/utils.hpp"

#include <cstdlib>
#include <sstream>

// sudo journalctl --output json-pretty -n 20 -u nginx
std::vector<palm::systemd::models::journal::Item> palm::systemd::logs(
    const std::string& service_name, bool user_scope, time_t since_,
    time_t until_) {
  const std::string since = palm::epoch_to_journald_timestamp(since_);
  const std::string until = palm::epoch_to_journald_timestamp(until_);
  spdlog::info("fetch logs for {} from {} to {}", service_name, since, until);

  std::vector<std::string> args = {"--output", "json", "--since", since,
                                   "--until",  until,  "-u",      service_name};
  if (user_scope) {
    args.push_back("--user");
  }
  const auto& [code, out, err] = palm::shell("/usr/bin/journalctl", args);
  if (code != EXIT_SUCCESS) {
    spdlog::error("{}", err);
    return {};
  }

  std::vector<std::string> lines;
  std::vector<palm::systemd::models::journal::Item> items;
  boost::split(lines, out, boost::is_any_of("\n"));
  for (auto& line : lines) {
    boost::trim(line);
    if (line.empty()) {
      continue;
    }
    spdlog::debug("receive systemd log: {}", line);
    const auto js = nlohmann::json::parse(line);
    const auto it = js.template get<palm::systemd::models::journal::Item>();
    items.push_back(it);
  }

  return items;
}
