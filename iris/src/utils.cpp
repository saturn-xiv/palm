#include "iris/utils.hpp"

#include <array>
#include <cctype>
#include <chrono>
#include <ctime>
#include <format>
#include <fstream>
#include <memory>
#include <random>
#include <stdexcept>
#include <streambuf>

#include <spdlog/spdlog.h>
#include <uuid.h>
#include <cppcodec/base64_rfc4648.hpp>
#include <cppcodec/base64_url.hpp>

std::string iris::timestamp(const std::string &prefix) {
  //   std::time_t tm = std::time(nullptr);
  //   std::chrono::system_clock::time_point now =
  //       std::chrono::system_clock::from_time_t(tm);
  const auto now = std::chrono::system_clock::now();
  const auto now_s = std::chrono::time_point_cast<std::chrono::seconds>(now);
  std::string it = std::format("{}-{:%Y%m%d%H%M%S}", prefix, now_s);
  return it;
}

std::string iris::uuid() {
  std::random_device rd;
  auto seed_data = std::array<int, std::mt19937::state_size>{};
  std::generate(std::begin(seed_data), std::end(seed_data), std::ref(rd));
  std::seed_seq seq(std::begin(seed_data), std::end(seed_data));
  std::mt19937 generator(seq);
  uuids::uuid_random_generator gen{generator};

  const auto it = gen();
  return uuids::to_string(it);
}

void iris::keep(const std::filesystem::path &folder, const std::string &prefix,
                size_t count) {
  // TODO
}

void iris::uncompress(const std::filesystem::path &folder,
                      const std::filesystem::path &tar) {
  // TODO
}
void iris::compress(const std::filesystem::path &folder) {
  // TODO
}

// static std::tuple<std::string, std::string, int> exec_shell_command(
//     const char *cmd) {
//   spdlog::debug("exec: {}", cmd);

//   std::array<char, 128> buffer;
//   std::stringstream ss;

//   std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
//   if (!pipe) {
//     throw std::runtime_error("popen() failed!");
//   }

//   while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
//     ss << buffer.data();
//   }

//   int exit_code = pclose(pipe.get());
//   const std::string result = ss.str();
//   spdlog::debug("exited: {} \n{}", WEXITSTATUS(exit_code), result);
//   spdlog::debug("aaa 0");
//   std::pair<std::string, int> it = std::make_pair(result, exit_code);
//   spdlog::debug("aaa 0.0");
//   return it;

// }

static inline std::string read_file_to_string(
    const std::filesystem::path &file) {
  std::ifstream it(file);
  std::string str((std::istreambuf_iterator<char>(it)),
                  std::istreambuf_iterator<char>());
  return str;
}
std::tuple<std::string, std::string, int> iris::execute(
    const std::vector<std::string> args) {
  std::stringstream ss;

  for (const auto &it : args) {
    ss << it << " ";
  }

  const auto tmp = std::filesystem::temp_directory_path();
  const auto id = iris::uuid();
  const auto out = tmp / std::format("{}.out", id);
  ss << ">" << out.string();
  const auto err = tmp / std::format("{}.err", id);
  ss << " 2>" << err.string();

  const std::string cmd = ss.str();
  spdlog::debug("exec: {}", cmd);
  const auto code = std::system(cmd.c_str());
  const auto out_s = read_file_to_string(out);
  const auto err_s = read_file_to_string(err);

  spdlog::debug("exited: {} ({}, {})", WEXITSTATUS(code), out_s, err_s);
  return {out_s, err_s, code};
}

int iris::md5(const std::filesystem::path &file_) {
  const auto file = file_.string();
  const std::string cmd = std::format("md5sum {} >{}.md5", file, file);
  return std::system(cmd.c_str());
}

void iris::check(const std::tuple<std::string, std::string, int> res) {
  if (std::get<2>(res) != EXIT_SUCCESS) {
    throw std::runtime_error(std::get<1>(res));
  }
}

bool iris::is_alphanumeric(const std::string &s) {
  return std::find_if(s.begin(), s.end(),
                      [](char c) { return !std::isalnum(c); }) == s.end();
}
