#include "aloe/postgresql.hpp"
#include "palm/crypto.hpp"
#include "palm/utils.hpp"

void aloe::PostgreSql::dump() {
  const std::string tmp =
      std::format("{}-{}", this->_db_name, palm::timestamp());
  const std::string tar = std::format("{}.tar", tmp);

  {
    spdlog::debug("create {}", tmp);
    std::filesystem::create_directories(tmp);
  }
  {
    const std::filesystem::path schema_sql =
        std::filesystem::path(tmp) / SCHEMA_SQL;
    const std::filesystem::path data_dump =
        std::filesystem::path(tmp) / DATA_DUMP;
    spdlog::info("dump postgresql://{}@{}:{}/{} to {}", this->_user,
                 this->_host, this->_port, this->_db_name, tmp);

    {
      const auto& [status, out, err] = palm::shell(
          "/usr/bin/pg_dump",
          {"-O", "-s", "-w", "-d", this->url(), "-f", schema_sql.string()});
      if (status != EXIT_SUCCESS) {
        spdlog::error("{} {}", status, err);
        return;
      }
      spdlog::debug("{}", out);
    }
    {
      const auto& [status, out, err] = palm::shell(
          "/usr/bin/pg_dump", {"-Fc", "-O", "-a", "-w", "-d", this->url(), "-f",
                               data_dump.string()});
      if (status != EXIT_SUCCESS) {
        spdlog::error("{} {}", status, err);
        return;
      }
      spdlog::debug("{}", out);
    }
  }
  {
    const auto& [status, out, err] = palm::tar(tar, tmp);
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
  spdlog::info("done.");
}

void aloe::PostgreSql::restore(const std::string& name) {
  {
    const auto& [status, out, err] = palm::shell(
        "/usr/bin/xz", {"--decompress", std::format("{}.tar.xz", name)});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  {
    const std::filesystem::path schema_sql =
        std::filesystem::path(name) / SCHEMA_SQL;
    const auto& [status, out, err] = palm::shell(
        "/usr/bin/psql", {"-d", this->url(), "-f", schema_sql.string()});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  {
    const std::filesystem::path data_dump =
        std::filesystem::path(name) / DATA_DUMP;
    const auto& [status, out, err] =
        palm::shell("/usr/bin/pg_restore",
                    {"-Fc", "-d", this->url(), "-f", data_dump.string()});
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    spdlog::debug("{}", out);
  }
  spdlog::info("done.");
}
