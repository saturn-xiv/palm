#include "aloe/oracle.hpp"
#include "palm/crypto.hpp"
#include "palm/utils.hpp"

void aloe::Oracle::dump() {
  const std::string tmp = std::format("{}-{}", this->_sid, palm::timestamp());
  const std::string tar = std::format("{}.tar", tmp);
  spdlog::info("dump oracle://{}@:127.0.0.1:1521/{} to {}/{}", this->_user,
               this->_sid, this->_directory_path, tmp);
  const std::string log = std::format("{}.exp.log", tmp);
  const std::string dmp = std::format("{}.dmp", tmp);

  {
    const auto& [status, out, err] = palm::shell(
        "/usr/bin/expdp",
        {std::format("{}/{}", this->_user, this->_password), "full=Y",
         std::format("dumpfile=", dmp), std::format("logfile=", log)});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }

  {
    const auto& [status, out, err] =
        palm::tar(tar, this->_directory_path, {log, dmp});
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

void aloe::Oracle::restore(const std::string& name, const std::string& user) {
  spdlog::info("restore {} to oracle://{}@:127.0.0.1:1521/{}", name,
               this->_user, this->_sid);
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
    std::filesystem::rename(dmp,
                            std::filesystem::path(this->_directory_path) / dmp);
  }
  {
    const auto& [status, out, err] = palm::shell(
        "/usr/bin/impdp",
        {std::format("{}/{}", this->_user, this->_password), "full=Y",
         std::format("dumpfile={}", dmp), std::format("logfile={}", log),
         std::format("schemas={}", this->_user),
         std::format("remap_schema={}:{}", this->_user, user)});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  {
    std::filesystem::path it =
        std::filesystem::path(this->_directory_path) / dmp;
    spdlog::debug("remove {}", it.string());
    std::filesystem::remove(it);
  }
  {
    const auto& [status, out, err] =
        palm::xz(std::filesystem::path(this->_directory_path) / log);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  spdlog::info("done.");
}
