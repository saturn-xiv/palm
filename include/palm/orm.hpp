#pragma once

#include "palm/env.hpp"

#include <SQLiteCpp/SQLiteCpp.h>

namespace palm {
// .show Displays current settings for various parameters
// .databases Provides database names and files
// .quit Quit sqlite3 program
// .tables Show current tables
// .schema Display schema of table
// .header Display or hide the output table header
// .mode Select mode for the output table
// .dump Dump database in SQL text format
// pragma compile_options;
// SELECT name FROM sqlite_master WHERE type='table' AND name='TABLE_NAME'
//
namespace sqlite {
std::shared_ptr<SQLite::Database> open(
    const std::filesystem::path& db, const bool wal_mode = true,
    const std::optional<std::chrono::seconds>& busy_timeout =
        std::chrono::seconds(5));
}
// https://www.postgresql.org/docs/current/runtime-config-logging.html
// /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
// SELECT VERSION() AS value
namespace postgresql {}
// use DB-NAME
// show tables;
// desc TABLE-NAME;
// SELECT table_name FROM information_schema.tables WHERE table_schema =
// 'databasename' AND table_name = 'testtable'; SHOW TABLES LIKE 'tablename';
// SELECT VERSION() AS value
namespace mysql {}
}  // namespace palm
