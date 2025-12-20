#include "iris/filesystem.hpp"
#include "iris/utils.hpp"

#include <spdlog/spdlog.h>

void iris::Filesystem::dump(const std::filesystem::path& output) const {
  const auto key = this->key_file();

  const auto& [out, err, code] = iris::execute(
      "rsync", "-avz", "-e",
      std::format("'ssh -p {}  -i {}'", this->_port, key.string()),
      std::format("{}@{}:{}", this->_user, this->_host, this->_folder),
      output.string());
  if (code != EXIT_SUCCESS) {
    throw std::runtime_error(err);
  }
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
