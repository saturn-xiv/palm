#include "aloe/dm8.hpp"
#include "palm/crypto.hpp"
#include "palm/utils.hpp"

void aloe::Dm8::dump(const std::string& directory) {
  const std::string tmp = std::format("{}-{}", this->_user, palm::timestamp());
  const std::string tar = std::format("{}.tar", tmp);
  spdlog::info("dump dm8://{}@:127.0.0.1:{} to {}/{}", this->_user, this->_port,
               directory, tmp);
  const std::string log = std::format("{}.exp.log", tmp);
  const std::string dmp = std::format("{}.dmp", tmp);

  {
    const auto& [status, out, err] =
        palm::shell("/usr/bin/dexp",
                    {std::format("userid={}/{}:{}", this->_user,
                                 this->_password, this->_port),
                     std::format("directory={}", directory), "full=y",
                     std::format("file=", dmp), std::format("logs=", log)});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }

  {
    const auto& [status, out, err] = palm::tar(tar, directory, {log, dmp});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  {
    const auto& [status, out, err] = palm::xz(tar);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  spdlog::info("done({}.xz).", tmp);
}

void aloe::Dm8::restore(const std::string& directory, const std::string& name) {
  spdlog::info("restore {} to dm8://{}@:127.0.0.1:{}", name, this->_user,
               this->_port);
  const std::string log = std::format("{}.imp.log", name);
  const std::string dmp = std::format("{}.dmp", name);
  {
    const auto& [status, out, err] = palm::shell(
        "/usr/bin/xz", {"--decompress", std::format("{}.tar.xz", name)});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
    std::filesystem::rename(dmp, std::filesystem::path(directory) / dmp);
  }
  {
    const auto& [status, out, err] = palm::shell(
        "/usr/bin/dimp",
        {std::format("userid={}/{}:{}", this->_user, this->_password,
                     this->_port),
         "full=y", std::format("file={}", dmp), std::format("logs={}", log),
         std::format("directory={}", directory)});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  {
    std::filesystem::path it = std::filesystem::path(directory) / dmp;
    spdlog::debug("remove {}", it.string());
    std::filesystem::remove(it);
  }
  {
    const auto& [status, out, err] =
        palm::xz(std::filesystem::path(directory) / log);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  spdlog::info("done.");
}
