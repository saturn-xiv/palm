#include "iris/minio.hpp"
#include "iris/utils.hpp"

#include <spdlog/spdlog.h>

void iris::Minio::dump(const std::filesystem::path& output) const {
  spdlog::info("backup {} to {}", this->_url, output.string());
  this->set_alias();
  const auto res = iris::execute(
      {"mc", "cp", "--quiet", "--recursive", this->_alias, output.string()});
  iris::check(res);
}

void iris::Minio::restore(const std::filesystem::path& input) const {
  spdlog::info("restore {} from {}", this->_url, input.string());
  this->set_alias();
  // TODO mc cp --quiet --recursive local-folder/ ALIAS
}

void iris::Minio::set_alias() const {
  const auto res =
      iris::execute({"mc", "alias", "set", this->_alias, this->_url,
                     this->_access_key, this->_secret_key});
  iris::check(res);
}
