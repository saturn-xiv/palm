#include "iris/application.hpp"
#include "iris/database.hpp"
#include "iris/filesystem.hpp"
#include "iris/minio.hpp"
#include "iris/utils.hpp"
#include "iris/version.hpp"

#include <algorithm>
#include <cstdlib>

#include <spdlog/spdlog.h>

static inline void keep_files(const std::string& prefix,
                              const std::string& suffix, size_t count) {
  std::vector<std::string> items;
  for (const auto& entry :
       std::filesystem::directory_iterator(std::filesystem::current_path())) {
    const auto path = entry.path();
    if (!std::filesystem::is_regular_file(path)) {
      continue;
    }
    {
      const auto name = path.filename().string();
      if (!name.starts_with(prefix)) {
        continue;
      }
      if (!name.ends_with(suffix)) {
        continue;
      }
    }
    const auto file = path.string();
    spdlog::debug("found file {}", file);
    items.push_back(file);
  }
  spdlog::info("found {} backups, will be keep {} records", items.size(),
               count);
  if (items.size() <= count) {
    return;
  }
  std::sort(items.begin(), items.end(), std::greater<std::string>());
  items.erase(items.begin(), items.begin() + count);

  for (const auto& it : items) {
    spdlog::warn("delete file {}", it);
    std::filesystem::remove(it);
    const std::string md5 = std::format("{}.md5", it);
    if (std::filesystem::exists(md5)) {
      spdlog::warn("delete file {}", md5);
      std::filesystem::remove(md5);
    }
  }
}

void iris::Application::dump(const std::string& input,
                             const std::string& output_, bool compress,
                             size_t keep) const {
  if (keep < 1) {
    throw std::invalid_argument("keep count should be more than one");
  }
  const auto output = std::filesystem::absolute(output_);
  const auto package = iris::timestamp(input);
  const std::filesystem::path root = output / package;
  if (std::filesystem::exists(root)) {
    const std::string err =
        std::format("folder {} already exists", root.string());
    throw std::invalid_argument(err);
  }
  spdlog::info("backup {} into {} and keep recent {} files", input,
               root.string(), keep);
  {
    spdlog::debug("create folder {}", root.string());
    std::filesystem::create_directories(root);

    const std::string config_file = std::format("{}.toml", input);
    spdlog::info("load source configuration from {}", config_file);
    const toml::table config = toml::parse_file(config_file);
    std::optional<std::string_view> type =
        config["type"].value<std::string_view>();
    std::shared_ptr<iris::Storage> it;
    if (type == std::nullopt) {
      throw std::invalid_argument("empty type item");
    } else if (type.value() == "sync") {
      it = std::make_shared<iris::Filesystem>(config);
    } else if (type.value() == "dm8") {
      it = std::make_shared<iris::Dm8>(config);
    } else if (type.value() == "minio") {
      it = std::make_shared<iris::Minio>(config);
    } else {
      const std::string msg =
          std::format("unsupported storage {}", type.value());
      throw std::invalid_argument(msg);
    }
    it->dump(root);
  }

  {
    std::filesystem::current_path(output);
    spdlog::debug("changed working directory to {}",
                  std::filesystem::current_path().string());
  }

  const std::string tar = std::format("{}.tar", package);
  {
    spdlog::debug("compressing {}", tar);
    const auto res =
        iris::execute({"tar", "--remove-files", "-cf", tar, package});
    iris::check(res);
  }

  if (compress) {
    const std::string zip = std::format("{}.xz", tar);
    {
      spdlog::debug("compressing {}", zip);
      const auto res = iris::execute(
          {"xz", "-z", "-F", "xz", "-C", "sha256", "--best", tar});
      iris::check(res);
    }
    iris::md5(zip);
    keep_files(input + "-", ".tar.xz", keep);
  } else {
    iris::md5(tar);
    keep_files(input + "-", ".tar", keep);
  }

  const std::string output_config = std::format("{}.toml", output_);
  if (std::filesystem::exists(output_config)) {
    spdlog::info("load destination configuration from {}", output_config);
    const toml::table config = toml::parse_file(output_config);
    iris::Filesystem it(config);
    it.upload(output);
  }
}
