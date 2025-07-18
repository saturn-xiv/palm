#include "palm/podman.hpp"

#include <sstream>

#include <boost/asio.hpp>
#include <boost/process.hpp>

#include <spdlog/spdlog.h>

// https://github.com/boostorg/process/blob/boost-1.88.0/example/stdio.cpp

// podman stats -a --format json --no-stream
// podman ps -a --format json
// podman logs --since "2025-07-17T01:00:00" --until "2025-07-18T02:00:00" -n -t
// 091c84e9e812

std::vector<palm::podman::models::Log> palm::podman::logs(
    const std::string& container_id, std::tm* begin,
    const std::chrono::seconds ttl) {
  // TODO
}

std::vector<palm::podman::models::Status> palm::podman::stats() {
  boost::asio::io_context ctx;
  boost::asio::readable_pipe out{ctx};
  std::string buf;

  boost::process::v2::process cmd(
      ctx, "/usr/bin/podman",
      {"stats", "-a", "--format", "json", "--no-stream"},
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
std::vector<palm::podman::models::container::Item> palm::podman::ps() {
  boost::asio::io_context ctx;
  boost::asio::readable_pipe out{ctx};
  std::string buf;

  boost::process::v2::process cmd(
      ctx, "/usr/bin/podman", {"ps", "-a", "--format", "json"},
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
