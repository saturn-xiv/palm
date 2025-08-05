#include "aloe/mysql.hpp"
#include "palm/crypto.hpp"
#include "palm/utils.hpp"

void aloe::MySql::dump() {
  const auto sql = std::format("{}-{}.sql", this->_db_name, palm::timestamp());
  {
    spdlog::info("dump mysql://{}@{}:{}/{} to {}", this->_user, this->_host,
                 this->_port, this->_db_name, sql);

    std::vector<std::string> args = {"-h", this->_host,
                                     "-P", std::to_string(this->_port),
                                     "-u", this->_user};
    if (this->_password) {
      args.push_back("-p");
      args.push_back(this->_password.value());
    }
    args.push_back("-D");
    args.push_back(this->_db_name);
    args.push_back("-r");
    args.push_back(sql);

    const auto& [status, out, err] = palm::shell("/usr/bin/mysqldump", args);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  {
    const auto& [status, out, err] = palm::xz(sql);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  spdlog::info("done({}.xz).", sql);
}

void aloe::MySql::restore(const std::string& name) {
  {
    const auto& [status, out, err] = palm::shell(
        "/usr/bin/xz", {"--decompress", std::format("{}.sql.xz", name)});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  {
    spdlog::info("restore {} to mysql://{}@{}:{}/{}", name, this->_user,
                 this->_host, this->_port, this->_db_name);

    std::vector<std::string> args = {"-h", this->_host,
                                     "-P", std::to_string(this->_port),
                                     "-u", this->_user};
    if (this->_password) {
      args.push_back("-p");
      args.push_back(this->_password.value());
    }
    args.push_back("-D");
    args.push_back(this->_db_name);
    args.push_back("-e");
    args.push_back(std::format("source {}.sql", name));
    const auto& [status, out, err] = palm::shell("/usr/bin/mysql", args);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  spdlog::info("done.");
}
