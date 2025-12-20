#include "iris/database.hpp"
#include "iris/utils.hpp"

#include <spdlog/spdlog.h>

// NLS_LANG=AMERICAN_AMERICA.AL32UTF8
void iris::Oracle::dump(const std::filesystem::path& file) const {
  // TODO expdp "{{ app_db_user }}/{{ app_db_password }}" full=Y
  // dumpfile=$VERSION.dmp logfile=$VERSION.log
}
void iris::Oracle::restore(const std::filesystem::path& file) const {
  // TODO impdp "{{ app_db_user }}/{{ app_db_password }}" dumpfile=$2.dmp
  // logfile=$VERSION.log schemas=$3 REMAP_SCHEMA=$3:$4
}
