#include "iris/filesystem.hpp"
#include "iris/utils.hpp"

#include <algorithm>

#include <spdlog/spdlog.h>

static inline bool is_local(const std::string& ip) {
  const static std::vector<std::string> items = {"127.0.0.1", "localhost"};
  return std::find(items.begin(), items.end(), ip) != items.end();
}
void iris::Filesystem::upload(const std::filesystem::path& folder) const {
  spdlog::info("upload {} to {}@{}:{}:{}", folder.string(), this->_user,
               this->_host, this->_port, this->_folder);
  const auto key = this->key_file();
  const auto res = iris::execute(
      {"rsync", "-az", "-e",
       std::format("'ssh -p {}  -i {}'", this->_port, key.string()),
       folder.string(),
       std::format("{}@{}:{}/", this->_user, this->_host, this->_folder)});
  iris::check(res);
}
// https://linux.die.net/man/1/rsync
void iris::Filesystem::dump(const std::filesystem::path& output_) const {
  spdlog::info("backup {}@{}:{}:{} to {}", this->_user, this->_host,
               this->_port, this->_folder, output_.string());
  const std::string output = std::format("{}/", output_.string());
  const std::string folder = std::format("{}/", this->_folder);
  if (is_local(this->_host)) {
    std::vector<std::string> args = {"rsync", "-az", folder};
    for (const auto it : this->_excludes) {
      args.push_back("--exclude");
      args.push_back(it);
    }
    args.push_back(output);
    const auto res = iris::execute(args);
    iris::check(res);
    return;
  }

  const auto key = this->key_file();

  std::vector<std::string> args = {
      "rsync", "-az", "-e",
      std::format("'ssh -p {}  -i {}'", this->_port, key.string()),
      std::format("{}@{}:{}", this->_user, this->_host, folder)};
  for (const auto it : this->_excludes) {
    args.push_back("--exclude");
    args.push_back(it);
  }
  args.push_back(output);
  const auto res = iris::execute(args);
  iris::check(res);
}
void iris::Filesystem::restore(const std::filesystem::path& file) const {
  // TODO
}

std::filesystem::path iris::Filesystem::key_file() const {
  if (this->_key_file) {
    return this->_key_file.value();
  }
  const auto home = iris::home();
  for (const auto it : {"id_ed25519", "id_rsa"}) {
    const auto file = home / ".ssh" / it;
    if (std::filesystem::is_regular_file(file)) {
      return file;
    }
  }
  throw std::invalid_argument("couldn't found the ssh private key file");
}
