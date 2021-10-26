#pragma once

#include "palm/forum.hpp"

namespace palm {
namespace fig {
namespace app {
void start_web();
void start_rpc();
void start_worker();
void db_migrate();
void db_rollback();
void db_status();
void cache_list();
void cache_clear();
void generate_config(const std::filesystem::path& file);
void generate_systemd(const std::string& domain);
void generate_nginx(const std::string& domain, const bool ssl);
void generate_db_migration(const std::string& name);
}  // namespace app
}  // namespace fig
}  // namespace palm
