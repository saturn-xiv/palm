#include "iris/database.hpp"
#include "iris/utils.hpp"

#include <spdlog/spdlog.h>

void iris::Dm8::dump(const std::filesystem::path& output) const {
  spdlog::info("backup {}@{}:{}  to {}", this->_user, this->_host, this->_port,
               output.string());
  {
    std::filesystem::current_path(this->_home);
    spdlog::debug("changed working directory to {}",
                  std::filesystem::current_path().string());
  }
  const auto res = iris::execute(
      {"./dexp",
       std::format("{}/{}@{}:{}", this->_user, this->_password.value_or(""),
                   this->_host, this->_port),
       std::format("directory={}", output.string()), "file=full.dmp",
       "log=dump.log", "full=y"});
  iris::check(res);
}
void iris::Dm8::restore(const std::filesystem::path& file) const {
  // TODO  ./dimp SYSDBA/$3@127.0.0.1:$2 file=full.dmp log=restore.log
  // directory=$4 full=y
}
