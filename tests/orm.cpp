#include <catch2/catch_test_macros.hpp>

#include "palm/orm.hpp"

#include <iostream>

TEST_CASE("by soci", "[postgresql]") {
  palm::PostgreSql config("127.0.0.1", 5432, "www", "change-me", "lavender");
  auto pool = config.open();
  SECTION("version") {
    soci::session db(*pool);
    std::string version;
    db << "SELECT VERSION()", soci::into(version);
    std::cout << version << std::endl;
  }
}

TEST_CASE("by soci", "[mysql]") {}

TEST_CASE("by soci", "[sqlite3]") {
  palm::Sqlite3 config("db.sqlite3");

  auto db = config.open();
  SECTION("version") {
    std::string version;
    *db << "SELECT SQLITE_VERSION()", soci::into(version);
    std::cout << "Sqlite3 Version: " << version << std::endl;
  }
}
