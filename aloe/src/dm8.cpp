#include "aloe/dm8.hpp"
#include "aloe/utils.hpp"
#include "palm/crypto.hpp"
#include "palm/utils.hpp"

void aloe::Dm8::dump(const std::string& directory, bool compress) {
  const std::string tmp = std::format("{}-{}", this->_user, palm::timestamp());
  const std::string tar = std::format("{}.tar", tmp);
  const auto zip = std::format("{}.tar.xz", tmp);
  const auto md5 = std::format("{}.md5", tmp);
  spdlog::info("dump dm8://{}@{}:{} to {}/{}", this->_user, this->_host,
               this->_port, directory, tmp);
  const std::string log = std::format("{}.exp.log", tmp);
  const std::string dmp = std::format("{}.dmp", tmp);

  {
    const auto cmd = this->_home / "bin" / "dexp";
    const auto& [status, out, err] = palm::shell(
        cmd.string(),
        {this->_url(), std::format("directory={}", directory), "full=y",
         "nolog=y", std::format("file={}", dmp), std::format("log={}", log)});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }

  {
    spdlog::info("create {}", tar);
    const auto& [status, out, err] = palm::tar(tar, directory, {log, dmp});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);

    {
      const auto hash = palm::md5(tar);
      std::ofstream out(md5);
      if (!out.is_open()) {
        spdlog::error("couldn't create md5 file");
        return;
      }
      out << hash.value() << " " << tar << std::endl;
      out.close();
    }
  }
  if (compress) {
    spdlog::info("create {}", zip);
    const auto& [status, out, err] = palm::xz(tar);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
    {
      const auto hash = palm::md5(zip);
      std::ofstream out(md5, std::ios::app);
      if (!out.is_open()) {
        spdlog::error("couldn't open md5 file");
        return;
      }
      out << hash.value() << zip << std::endl;
      out.close();
    }
  }
  spdlog::info("done({}).", tmp);
}

void aloe::Dm8::restore(const std::string& directory,
                        const std::filesystem::path& file) {
  spdlog::info("restore {} to dm8://{}:{}:{}", file.string(), this->_user,
               this->_host, this->_port);

  const std::string name = aloe::filename(file);
  const std::string log = std::format("{}.imp.log", name);
  const std::string dmp = std::format("{}.dmp", name);
  {
    const auto& [status, out, err] =
        palm::shell("/usr/bin/tar", {"xf", std::format("{}.tar.xz", name)});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
    std::filesystem::rename(dmp, std::filesystem::path(directory) / dmp);
  }
  {
    const auto cmd = this->_home / "bin" / "dimp";
    const auto& [status, out, err] = palm::shell(
        cmd.string(),
        {this->_url(), "full=y", "nolog=y", std::format("file={}", dmp),
         std::format("log={}", log), std::format("directory={}", directory)});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  spdlog::info("done.");
}
