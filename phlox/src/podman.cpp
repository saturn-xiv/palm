#include "palm/podman.hpp"

#include <sstream>

#include <boost/asio.hpp>
#include <boost/process.hpp>

#include <spdlog/spdlog.h>

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
std::vector<palm::podman::models::Log> palm::podman::logs(
    const std::string& container_id, std::tm* since_, std::tm* until_) {
  const std::string since = "";
  const std::string until = "";
  spdlog::debug("fetch logs for {} from {} to {}", container_id, since, until);
  std::vector<palm::podman::models::Log> items;
  // TODO
  return items;
}

std::vector<palm::podman::models::Status> palm::podman::stats(bool all) {
  boost::asio::io_context ctx;
  boost::asio::readable_pipe out{ctx};
  std::string buf;

  std::vector<std::string> args = {"stats", "--format", "json", "--no-stream"};
  if (all) {
    args.push_back("-a");
  }

  boost::process::v2::process cmd(
      ctx, "/usr/bin/podman", args,
      boost::process::v2::process_stdio{{}, out, {}});
  boost::system::error_code ec;
  boost::asio::read(out, boost::asio::dynamic_buffer(buf), ec);
  if (ec && ec != boost::asio::error::eof) {
    spdlog::error("{}", ec.message());
    return {};
  }
  cmd.wait();

  const auto js = nlohmann::json::parse(buf);
  std::vector<palm::podman::models::Status> items = js;
  return items;
}
std::vector<palm::podman::models::container::Item> palm::podman::ps(bool all) {
  boost::asio::io_context ctx;
  boost::asio::readable_pipe out{ctx};
  std::string buf;

  std::vector<std::string> args = {"ps", "-a", "--format", "json"};
  if (all) {
    args.push_back("-a");
  }
  boost::process::v2::process cmd(
      ctx, "/usr/bin/podman", args,
      boost::process::v2::process_stdio{{}, out, {}});
  boost::system::error_code ec;
  boost::asio::read(out, boost::asio::dynamic_buffer(buf), ec);
  if (ec && ec != boost::asio::error::eof) {
    spdlog::error("{}", ec.message());
    return {};
  }
  cmd.wait();

  const auto js = nlohmann::json::parse(buf);
  std::vector<palm::podman::models::container::Item> items = js;
  return items;
}
