#pragma once

#include <filesystem>
#include <format>
#include <string>
#include <vector>

#include <boost/algorithm/string.hpp>
#include <boost/asio.hpp>
#include <boost/process.hpp>
#include <boost/type_index.hpp>

#include <spdlog/spdlog.h>

namespace palm {

void init(bool debug);

inline int64_t epoch_in_seconds() {
  const auto now = std::chrono::system_clock::now();
  const auto epoch = now.time_since_epoch();
  const auto seconds = std::chrono::duration_cast<std::chrono::seconds>(epoch);
  return seconds.count();
}

inline std::string truncate(const std::string& s, uint l,
                            const std::string& ellipse = "...") {
  int c = l - ellipse.length();
  if (c <= 0 || s.length() <= l) {
    return s;
  }
  return boost::algorithm::erase_tail_copy(s, c) + ellipse;
}

void load(const std::filesystem::path& f, std::string& s);
void load(const std::filesystem::path& f, std::vector<uint8_t> s);

inline bool is_stopped() {
  static const std::string file = ".stop";
  const auto ok = std::filesystem::exists(file);
  if (ok) {
    spdlog::warn("file {} exists, will be exited...", file);
  }
  return ok;
}

inline std::tuple<int, std::string, std::string> shell(
    const std::string& command, const std::vector<std::string>& args) {
  boost::asio::io_context ctx;

  boost::asio::readable_pipe out_p(ctx);
  boost::asio::readable_pipe err_p(ctx);

  boost::process::v2::process proc(
      ctx, command, args, boost::process::v2::process_stdio{{}, out_p, err_p});

  std::string out;
  std::string err;

  {
    boost::system::error_code ec;
    boost::asio::read(out_p, boost::asio::dynamic_buffer(out), ec);
    if (ec && ec != boost::asio::error::eof) {
      spdlog::error("read stdout: {}", ec.message());
      return {};
    }
  }
  {
    boost::system::error_code ec;
    boost::asio::read(err_p, boost::asio::dynamic_buffer(err), ec);
    if (ec && ec != boost::asio::error::eof) {
      spdlog::error("read stderr: {}", ec.message());
      return {};
    }
  }

  const int exit_code = proc.wait();

  return {exit_code, out, err};
}

namespace gravatar {
std::string hash(const std::string& email);
// https://docs.gravatar.com/api/avatars/images/
inline std::string image(const std::string& email) {
  const auto h = hash(email);
  return std::format("https://gravatar.com/avatar/{}", h);
}
// https://docs.gravatar.com/api/profiles/
inline std::string profile(const std::string& email) {
  const auto h = hash(email);
  return std::format("https://gravatar.com/{}.json", h);
}
}  // namespace gravatar

}  // namespace palm
